use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use nero_extensions::{WasmExtension, WasmHost};
use percent_encoding::percent_decode_str;
use url::Url;

use crate::error::{Error, Result};
use crate::interactions;

use super::keyvalue::FileStoreBackend;
use super::{cache_key, portal, CacheLimit, ExtensionId, ExtensionMetadata, Options};

#[derive(Clone)]
pub struct LoadedExtension {
    pub id: ExtensionId,
    pub file_path: PathBuf,
    pub options: Options,
    pub extension: Arc<WasmExtension>,
    callbacks: Arc<portal::Callbacks>,
}

pub struct Registry {
    host: WasmHost,
    transport: Arc<interactions::Transport>,
    loaded: RwLock<BTreeMap<ExtensionId, LoadedExtension>>,
}

impl Registry {
    pub fn new(transport: Arc<interactions::Transport>) -> Self {
        Self {
            host: WasmHost::default(),
            transport,
            loaded: RwLock::default(),
        }
    }

    pub async fn inspect(file_path: impl AsRef<Path>) -> Result<Arc<ExtensionMetadata>> {
        let metadata = WasmHost::inspect(file_path)
            .await
            .map_err(|error| Error::extension("inspect component", error))?;

        ExtensionId::try_from(&metadata)?;

        Ok(Arc::new(metadata))
    }

    pub async fn add(&self, file_path: PathBuf, options: Options) -> Result<LoadedExtension> {
        let file_path = std::fs::canonicalize(&file_path)
            .map_err(|error| Error::storage("resolve extension path", error))?;
        let metadata = WasmHost::inspect(&file_path)
            .await
            .map_err(|error| Error::extension("inspect component", error))?;
        let id = ExtensionId::try_from(&metadata)?;

        if self.contains(&id) {
            return Err(Error::ExtensionAlreadyLoaded(id.to_string()));
        }

        let keyvalue = Arc::new(
            FileStoreBackend::new(
                options.cache_dir.join(cache_key(&id)),
                options.max_cache_size.map(CacheLimit::bytes),
            )
            .await?,
        );
        let interaction = Arc::new(portal::Interaction::new(id.clone(), self.transport.clone()));
        let opener = Arc::new(portal::Opener);
        let callbacks = Arc::new(portal::Callbacks::new(&id));

        let extension = self
            .host
            .load_extension_async(&file_path, keyvalue, interaction, opener, callbacks.clone())
            .await
            .map_err(|error| Error::extension("load component", error))?;

        let entry = LoadedExtension {
            id: id.clone(),
            file_path,
            options,
            extension: Arc::new(extension),
            callbacks,
        };

        self.loaded.write().unwrap().insert(id, entry.clone());

        Ok(entry)
    }

    pub fn remove(&self, id: &ExtensionId) -> Option<LoadedExtension> {
        self.loaded.write().unwrap().remove(id)
    }

    pub fn contains(&self, id: &ExtensionId) -> bool {
        self.loaded.read().unwrap().contains_key(id)
    }

    pub fn get(&self, id: &ExtensionId) -> Option<LoadedExtension> {
        self.loaded.read().unwrap().get(id).cloned()
    }

    pub fn values(&self) -> Vec<LoadedExtension> {
        self.loaded.read().unwrap().values().cloned().collect()
    }

    pub fn deliver_callback(&self, uri: String) -> Result<()> {
        let mut callback =
            Url::parse(&uri).map_err(|error| Error::InvalidCallbackUrl(error.to_string()))?;

        if callback.scheme() != "nero"
            || callback.host_str() != Some("callback")
            || !callback.username().is_empty()
            || callback.password().is_some()
            || callback.port().is_some()
        {
            return Err(Error::InvalidCallbackUrl(uri));
        }

        callback.set_query(None);
        callback.set_fragment(None);

        let address = callback.to_string();

        let mut segments = callback
            .path_segments()
            .ok_or_else(|| Error::InvalidCallbackUrl(uri.clone()))?;
        let segment = segments
            .next()
            .filter(|segment| !segment.is_empty())
            .ok_or_else(|| Error::InvalidCallbackUrl(uri.clone()))?;

        if segments.next().is_some() {
            return Err(Error::InvalidCallbackUrl(uri));
        }

        let id = ExtensionId(
            percent_decode_str(segment)
                .decode_utf8()
                .map_err(|error| Error::InvalidCallbackUrl(error.to_string()))?
                .into_owned(),
        );
        let callbacks = self
            .loaded
            .read()
            .unwrap()
            .get(&id)
            .map(|extension| extension.callbacks.clone())
            .ok_or_else(|| Error::CallbackNotPending(address.clone()))?;

        if callbacks.address != address {
            return Err(Error::CallbackNotPending(address));
        }

        callbacks.deliver(uri)
    }
}

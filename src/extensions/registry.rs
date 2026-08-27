use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use nero_extensions::{WasmExtension, WasmHost};

use crate::error::{Error, Result};

use super::keyvalue::FileStoreBackend;
use super::{cache_key, portal, CacheLimit, ExtensionId, ExtensionMetadata, Options};

#[derive(Clone)]
pub struct LoadedExtension {
    pub id: ExtensionId,
    pub file_path: PathBuf,
    pub options: Options,
    pub extension: Arc<WasmExtension>,
}

#[derive(Default)]
pub struct Registry {
    host: WasmHost,
    loaded: RwLock<BTreeMap<ExtensionId, LoadedExtension>>,
}

impl Registry {
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
        let interaction = Arc::new(portal::Interaction);
        let opener = Arc::new(portal::Opener);
        let callbacks = Arc::new(portal::Callbacks);

        let extension = self
            .host
            .load_extension_async(&file_path, keyvalue, interaction, opener, callbacks)
            .await
            .map_err(|error| Error::extension("load component", error))?;

        let entry = LoadedExtension {
            id: id.clone(),
            file_path,
            options,
            extension: Arc::new(extension),
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
}

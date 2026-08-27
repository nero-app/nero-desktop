use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use blake3::{Hash, Hasher};
use iced::widget::image;
use iced::Task;
use nero_extensions::types::MediaResource;
use nero_file_store::FileStore;
use reqwest::Client;

use crate::error::{Error, Result};

#[derive(Clone)]
struct DiskCache {
    store: Arc<FileStore>,
}

impl DiskCache {
    async fn new(cache_dir: PathBuf) -> Result<Self> {
        let store = tokio::task::spawn_blocking(move || FileStore::new(cache_dir, None))
            .await
            .map_err(|error| Error::task("open image cache", error))?
            .map_err(|error| Error::storage("open image cache", error))?;

        Ok(Self {
            store: Arc::new(store),
        })
    }

    async fn get(&self, hash: Hash) -> Option<Vec<u8>> {
        let store = self.store.clone();
        let key = hash.to_hex().to_string();

        match tokio::task::spawn_blocking(move || store.get(&key)).await {
            Ok(Ok(bytes)) => bytes,
            Ok(Err(error)) => {
                tracing::warn!(%error, "failed to read image from disk cache");
                None
            }
            Err(error) => {
                tracing::warn!(%error, "image disk cache task failed");
                None
            }
        }
    }

    async fn insert(&self, hash: Hash, bytes: Vec<u8>) {
        let store = self.store.clone();
        let key = hash.to_hex().to_string();
        let write = tokio::task::spawn_blocking(move || store.set(&key, bytes, None)).await;

        match write {
            Ok(Ok(())) => {}
            Ok(Err(error)) => {
                tracing::warn!(%error, "failed to write image to disk cache");
            }
            Err(error) => {
                tracing::warn!(%error, "image disk cache task failed");
            }
        }
    }
}

#[derive(Clone)]
pub struct Images {
    client: Client,
    cache: DiskCache,
    entries: Arc<Mutex<HashMap<Hash, Entry>>>,
}

enum Entry {
    Loading,
    Loaded(image::Handle),
    Failed,
}

impl Images {
    pub async fn new(client: Client, cache_dir: PathBuf) -> Result<Self> {
        Ok(Self {
            client,
            cache: DiskCache::new(cache_dir).await?,
            entries: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub fn load<Message>(&self, resource: MediaResource, loaded: Message) -> Task<Message>
    where
        Message: Clone + Send + 'static,
    {
        self.load_all([resource], loaded)
    }

    pub fn load_all<Message>(
        &self,
        resources: impl IntoIterator<Item = MediaResource>,
        loaded: Message,
    ) -> Task<Message>
    where
        Message: Clone + Send + 'static,
    {
        Task::batch(resources.into_iter().filter_map(|resource| {
            self.start(resource).map(|task| {
                let loaded = loaded.clone();
                task.map(move |_| loaded.clone())
            })
        }))
    }

    pub fn handle(&self, resource: &MediaResource) -> Option<image::Handle> {
        let hash = hash(resource)?;
        let entries = self.entries.lock().expect("image cache lock poisoned");

        match entries.get(&hash) {
            Some(Entry::Loaded(handle)) => Some(handle.clone()),
            _ => None,
        }
    }

    fn start(&self, resource: MediaResource) -> Option<Task<()>> {
        let hash = hash(&resource)?;

        {
            let mut entries = self.entries.lock().expect("image cache lock poisoned");

            if entries.contains_key(&hash) {
                return None;
            }

            entries.insert(hash, Entry::Loading);
        }

        let client = self.client.clone();
        let cache = self.cache.clone();
        let entries = self.entries.clone();
        let task = Task::perform(
            async move {
                let entry = match load_image(&client, &cache, resource, hash).await {
                    Ok(handle) => Entry::Loaded(handle),
                    Err(error) => {
                        tracing::warn!(%hash, %error, "failed to load image");
                        Entry::Failed
                    }
                };

                entries
                    .lock()
                    .expect("image cache lock poisoned")
                    .insert(hash, entry);
            },
            |_| (),
        );

        Some(task)
    }
}

async fn load_image(
    client: &Client,
    cache: &DiskCache,
    resource: MediaResource,
    hash: Hash,
) -> Result<image::Handle> {
    if let Some(bytes) = cache.get(hash).await {
        return Ok(image::Handle::from_bytes(bytes));
    }

    let bytes = fetch(client, resource).await?;
    cache.insert(hash, bytes.clone()).await;

    Ok(image::Handle::from_bytes(bytes))
}

async fn fetch(client: &Client, resource: MediaResource) -> Result<Vec<u8>> {
    let MediaResource::HttpRequest(request) = resource else {
        return Err(Error::UnsupportedImageResource);
    };
    let (parts, body) = request.into_parts();
    let mut request = client
        .request(parts.method, parts.uri.to_string())
        .headers(parts.headers);

    if let Some(body) = body {
        request = request.body(body);
    }

    let response = request
        .send()
        .await
        .map_err(|error| Error::http("fetching an image", error))?
        .error_for_status()
        .map_err(|error| Error::http("fetching an image", error))?;
    Ok(response
        .bytes()
        .await
        .map_err(|error| Error::http("reading an image response", error))?
        .to_vec())
}

fn hash(resource: &MediaResource) -> Option<Hash> {
    let MediaResource::HttpRequest(request) = resource else {
        return None;
    };
    let method = request.method().as_str().as_bytes();
    let uri = request.uri().to_string();
    let version = format!("{:?}", request.version());
    let mut headers = request.headers().iter().collect::<Vec<_>>();
    headers.sort_unstable_by(|(left_name, left_value), (right_name, right_value)| {
        left_name
            .as_str()
            .cmp(right_name.as_str())
            .then_with(|| left_value.as_bytes().cmp(right_value.as_bytes()))
    });

    let mut hasher = Hasher::new();
    hasher.update(&(method.len() as u64).to_le_bytes());
    hasher.update(method);
    hasher.update(&(uri.len() as u64).to_le_bytes());
    hasher.update(uri.as_bytes());
    hasher.update(&(version.len() as u64).to_le_bytes());
    hasher.update(version.as_bytes());
    hasher.update(&(headers.len() as u64).to_le_bytes());

    for (name, value) in headers {
        hasher.update(&(name.as_str().len() as u64).to_le_bytes());
        hasher.update(name.as_str().as_bytes());
        hasher.update(&(value.as_bytes().len() as u64).to_le_bytes());
        hasher.update(value.as_bytes());
    }

    match request.body() {
        Some(body) => {
            hasher.update(&[1]);
            hasher.update(&(body.len() as u64).to_le_bytes());
            hasher.update(body);
        }
        None => {
            hasher.update(&[0]);
        }
    }

    Some(hasher.finalize())
}

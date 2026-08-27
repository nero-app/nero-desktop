use std::{path::PathBuf, sync::Arc};

use nero_extensions::{Bucket, KeyResponse, Store, StoreError};
use nero_file_store::{Error as FileStoreError, FileStore};
use tokio::task::spawn_blocking;

use crate::error::{Error, Result};

pub struct FileStoreBackend {
    store: Arc<FileStore>,
}

impl FileStoreBackend {
    pub async fn new(root: PathBuf, max_bytes: Option<u64>) -> Result<Self> {
        let store = spawn_blocking(move || FileStore::new(root, max_bytes))
            .await
            .map_err(|error| Error::task("open extension cache", error))?
            .map_err(|error| Error::storage("open extension cache", error))?;

        Ok(Self {
            store: Arc::new(store),
        })
    }
}

#[async_trait::async_trait]
impl Store for FileStoreBackend {
    async fn open(&self, identifier: &str) -> std::result::Result<Arc<dyn Bucket>, StoreError> {
        if !identifier.is_empty() {
            return Err(StoreError::NoSuchBucket);
        }

        Ok(Arc::new(FileStoreBucket {
            store: self.store.clone(),
        }))
    }
}

struct FileStoreBucket {
    store: Arc<FileStore>,
}

#[async_trait::async_trait]
impl Bucket for FileStoreBucket {
    async fn get(&self, key: &str) -> std::result::Result<Option<Vec<u8>>, StoreError> {
        let store = self.store.clone();
        let key = key.to_owned();
        blocking(move || store.get(&key)).await
    }

    async fn set(
        &self,
        key: &str,
        value: Vec<u8>,
        ttl_ms: Option<u32>,
    ) -> std::result::Result<(), StoreError> {
        let store = self.store.clone();
        let key = key.to_owned();
        blocking(move || store.set(&key, value, ttl_ms)).await
    }

    async fn delete(&self, key: &str) -> std::result::Result<(), StoreError> {
        let store = self.store.clone();
        let key = key.to_owned();
        blocking(move || store.delete(&key)).await
    }

    async fn exists(&self, key: &str) -> std::result::Result<bool, StoreError> {
        let store = self.store.clone();
        let key = key.to_owned();
        blocking(move || store.exists(&key)).await
    }

    async fn list_keys(
        &self,
        cursor: Option<&str>,
    ) -> std::result::Result<KeyResponse, StoreError> {
        let store = self.store.clone();
        let cursor = cursor.map(str::to_owned);
        let (keys, cursor) = blocking(move || store.list_keys(cursor.as_deref())).await?;

        Ok(KeyResponse { keys, cursor })
    }
}

async fn blocking<T, F>(operation: F) -> std::result::Result<T, StoreError>
where
    F: FnOnce() -> std::result::Result<T, FileStoreError> + Send + 'static,
    T: Send + 'static,
{
    match spawn_blocking(operation).await {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(error)) => Err(match error {
            FileStoreError::StorageLimitExceeded => StoreError::StorageLimitExceeded,
            FileStoreError::Io(error) => StoreError::Other(error.to_string()),
            FileStoreError::Corrupt(message) => StoreError::Other(message),
        }),
        Err(error) => Err(StoreError::Other(error.to_string())),
    }
}

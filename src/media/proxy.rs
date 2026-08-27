use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result as AnyResult;
use axum::body::Body;
use axum::http::{Request, Response};
use librqbit::{Session, SessionOptions};
use nero_media_proxy::resources::{MediaReference, Resource};
use nero_media_proxy::torrent::librqbit::RqbitTorrentBackend;
use nero_media_proxy::torrent::{
    AddTorrentOptions, Torrent, TorrentBackend, TorrentFile, TorrentSource,
};
use nero_media_proxy::{MediaProxy, TorrentConfig};
use reqwest::Client;
use tokio::sync::{Mutex, RwLock};

use crate::error::{Error, Result};

use super::torrent::{TorrentResolver, TorrentResolverHandle};

pub fn default_torrent_dir() -> PathBuf {
    dirs::download_dir()
        .unwrap_or_else(std::env::temp_dir)
        .join("nero")
}

struct TorrentRuntime {
    output_dir: PathBuf,
    session: Arc<Session>,
    backend: Arc<dyn TorrentBackend>,
}

#[derive(Default)]
struct DynamicTorrentBackend {
    runtime: RwLock<Option<TorrentRuntime>>,
}

#[derive(Default)]
struct Configuration {
    playback_is_torrent: bool,
}

impl DynamicTorrentBackend {
    async fn backend(&self) -> AnyResult<Arc<dyn TorrentBackend>> {
        self.runtime
            .read()
            .await
            .as_ref()
            .map(|runtime| runtime.backend.clone())
            .ok_or_else(|| anyhow::anyhow!("torrent playback is disabled"))
    }

    async fn output_dir(&self) -> Option<PathBuf> {
        self.runtime
            .read()
            .await
            .as_ref()
            .map(|runtime| runtime.output_dir.clone())
    }

    async fn replace(&self, runtime: Option<TorrentRuntime>) -> Option<TorrentRuntime> {
        std::mem::replace(&mut *self.runtime.write().await, runtime)
    }
}

#[async_trait::async_trait]
impl TorrentBackend for DynamicTorrentBackend {
    async fn list_files(&self, source: &TorrentSource) -> AnyResult<Vec<TorrentFile>> {
        self.backend().await?.list_files(source).await
    }

    async fn add_torrent(
        &self,
        source: TorrentSource,
        options: Option<AddTorrentOptions>,
    ) -> AnyResult<Torrent> {
        self.backend().await?.add_torrent(source, options).await
    }

    async fn handle_stream_request(
        &self,
        torrent_id: &str,
        file_index: usize,
        request: Request<Body>,
    ) -> AnyResult<Response<Body>> {
        self.backend()
            .await?
            .handle_stream_request(torrent_id, file_index, request)
            .await
    }

    async fn cancel_torrent(&self, torrent: &str) -> AnyResult<()> {
        self.backend().await?.cancel_torrent(torrent).await
    }
}

#[derive(Clone)]
pub struct Media {
    proxy: Arc<MediaProxy>,
    backend: Arc<DynamicTorrentBackend>,
    resolver: Arc<TorrentResolverHandle>,
    client: Client,
    configuration: Arc<Mutex<Configuration>>,
}

impl Media {
    pub async fn new(
        client: Client,
        torrents_enabled: bool,
        torrent_output_dir: Option<PathBuf>,
    ) -> Result<Self> {
        let backend = Arc::new(DynamicTorrentBackend::default());
        let resolver = Arc::new(TorrentResolverHandle::default());
        let config = TorrentConfig {
            torrent_backend: backend.clone(),
            torrent_file_selector: Some(resolver.clone()),
        };
        let media = Self {
            proxy: Arc::new(MediaProxy::new(client.clone(), config)),
            backend,
            resolver,
            client,
            configuration: Arc::new(Mutex::new(Configuration::default())),
        };

        media
            .configure(torrents_enabled, torrent_output_dir)
            .await?;

        Ok(media)
    }

    pub fn proxy(&self) -> &Arc<MediaProxy> {
        &self.proxy
    }

    pub async fn register_playback(
        &self,
        resource: Resource,
        resolver: Option<TorrentResolver>,
    ) -> Result<MediaReference> {
        let mut configuration = self.configuration.lock().await;
        let is_torrent = matches!(&resource, Resource::Torrent(_));

        if is_torrent {
            if self.backend.output_dir().await.is_none() {
                return Err(Error::TorrentDisabled);
            }

            let resolver = resolver
                .ok_or_else(|| Error::Torrent("torrent resolver is missing".to_string()))?;
            self.resolver.set(Arc::new(resolver)).await;
        }

        let reference = self.proxy.register("playback", resource).await;
        configuration.playback_is_torrent = is_torrent;

        Ok(reference)
    }

    pub async fn configure(
        &self,
        torrents_enabled: bool,
        torrent_output_dir: Option<PathBuf>,
    ) -> Result<()> {
        let mut configuration = self.configuration.lock().await;
        let output_dir = torrent_output_dir.unwrap_or_else(default_torrent_dir);
        let current_output_dir = self.backend.output_dir().await;

        if current_output_dir.as_ref() == torrents_enabled.then_some(&output_dir) {
            return Ok(());
        }

        let runtime = if torrents_enabled {
            let session = Session::new_with_opts(
                output_dir.clone(),
                SessionOptions {
                    disable_dht_persistence: true,
                    ..SessionOptions::default()
                },
            )
            .await
            .map_err(|error| Error::Torrent(format!("{error:#}")))?;
            let backend = Arc::new(RqbitTorrentBackend::new(
                session.clone(),
                self.client.clone(),
            ));

            Some(TorrentRuntime {
                output_dir,
                session,
                backend,
            })
        } else {
            None
        };

        if configuration.playback_is_torrent {
            configuration.playback_is_torrent = false;
            self.proxy.unregister("playback").await;
        }

        if let Some(previous) = self.backend.replace(runtime).await {
            previous.session.stop().await;
        }

        Ok(())
    }
}

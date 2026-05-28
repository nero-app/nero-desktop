mod extensions;
mod preferences;

use std::{collections::HashMap, net::SocketAddr, path::PathBuf, sync::Arc};

use libnero::{Extension, ExtensionHost, types::ExtensionOptions};
use librqbit::Session;
use nero_media_proxy::{
    MediaProxy, MediaProxyConfig,
    torrent::{TorrentBackend, librqbit::RqbitTorrentBackend},
};
use reqwest::Client;
use tauri::{
    Manager, Runtime,
    plugin::{self, TauriPlugin},
};

use tauri::{Emitter, Result, async_runtime::RwLock};
use tokio::net::TcpListener;
use uuid::Uuid;

use crate::preferences::PreferencesData;

struct PluginState {
    host: ExtensionHost,
    extensions: RwLock<HashMap<Uuid, Extension>>,
}

impl PluginState {
    async fn new(data: &PreferencesData, proxy_addr: SocketAddr) -> Result<Self> {
        let http_client = Client::new();

        let torrent_backend = if let Some(torrent) = data.media_proxy.as_ref()
            && torrent.torrent_enabled
        {
            let output_dir = PathBuf::from(&torrent.torrent_output_folder);
            let librqbit_session = Session::new(output_dir).await?;
            let backend = RqbitTorrentBackend::new(librqbit_session, http_client.clone());
            Some(Arc::new(backend) as Arc<dyn TorrentBackend + 'static>)
        } else {
            None
        };

        let config = MediaProxyConfig {
            torrent_backend,
            // TODO: torrent_file_selector
            ..Default::default()
        };
        let proxy = MediaProxy::new(proxy_addr, http_client, config);

        let host = ExtensionHost::new(proxy);

        let mut extensions = HashMap::new();
        for extension in &data.extensions {
            let options = ExtensionOptions {
                cache_dir: PathBuf::from(&extension.options.cache_dir),
                max_cache_size: extension.options.max_cache_size,
            };
            match host.load(&extension.file_path, options).await {
                Ok(ext) => {
                    extensions.insert(extension.id, ext);
                }
                Err(e) => tracing::warn!(
                    "Failed to load extension {} on startup: {e}",
                    extension.file_path
                ),
            }
        }

        Ok(Self {
            host,
            extensions: RwLock::new(extensions),
        })
    }
}

pub fn init<R: Runtime>(proxy_addr: SocketAddr) -> TauriPlugin<R> {
    plugin::Builder::new("nero-extensions")
        .setup(move |app, _| {
            let preferences = PreferencesData::new(app);
            let app_handle = app.clone();
            tauri::async_runtime::spawn(async move {
                let state = PluginState::new(&preferences, proxy_addr).await.unwrap();

                let proxy = state.host.media_proxy();
                let app = proxy.router();

                tauri::async_runtime::spawn(async move {
                    let listener = TcpListener::bind(proxy_addr).await.unwrap();
                    axum::serve(listener, app).await
                });

                app_handle.manage(RwLock::new(preferences));
                app_handle.manage(state);
                app_handle.emit("nero-extensions://ready", ()).unwrap();
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            preferences::get_media_proxy_preferences,
            preferences::set_media_proxy_preferences,
            extensions::get_extension_metadata,
            extensions::load_extension,
            extensions::unload_extension,
            extensions::get_loaded_extensions,
            extensions::get_filters,
            extensions::search,
            extensions::get_series_info,
            extensions::get_series_episodes,
            extensions::get_series_videos,
        ])
        .build()
}

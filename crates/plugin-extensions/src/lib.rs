mod extensions;

use std::{net::SocketAddr, sync::Arc};

use libnero::{Extension, ExtensionHost, ExtensionMetadata};
use librqbit::Session;
use nero_processor::{Processor, torrent::RqbitTorrentBackend};
use reqwest::Client;
use tauri::{
    Manager, Runtime,
    plugin::{self, TauriPlugin},
};

use serde::Serialize;
use tauri::{AppHandle, Emitter, Result, State, async_runtime::RwLock};

#[derive(Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ExtensionInfo {
    file_path: String,

    metadata: Arc<ExtensionMetadata>,
    cache_dir: String,
    max_cache_size: Option<u64>,
}

#[derive(Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct PluginStatus {
    extension: Option<ExtensionInfo>,
}

struct PluginState {
    host: ExtensionHost,
    extension: RwLock<Option<Extension>>,
    status: RwLock<PluginStatus>,
}

impl PluginState {
    fn new(host: ExtensionHost) -> Self {
        Self {
            host,
            extension: Default::default(),
            status: Default::default(),
        }
    }

    async fn emit_status<R: Runtime>(&self, app: &AppHandle<R>) -> Result<()> {
        let status = self.status.read().await.clone();
        app.emit("nero-extensions://status-changed", status)
    }
}

#[tauri::command]
async fn get_status(state: State<'_, PluginState>) -> Result<PluginStatus> {
    Ok(state.status.read().await.clone())
}

pub struct Builder {
    processor_addr: SocketAddr,
}

impl Builder {
    pub fn new(processor_addr: SocketAddr) -> Self {
        Self { processor_addr }
    }

    pub fn build<R: Runtime>(self) -> TauriPlugin<R> {
        let processor_addr = self.processor_addr;

        // TODO: load torrent backend config from preferences
        plugin::Builder::new("nero-extensions")
            .setup(move |app, _| {
                let http_client = Client::new();
                let app_handle = app.app_handle().clone();

                tauri::async_runtime::spawn(async move {
                    let cache_dir = app_handle.path().app_cache_dir().unwrap();
                    let torrent_backend = Session::new(cache_dir).await.ok().map(|session| {
                        Arc::new(RqbitTorrentBackend::new(session, http_client.clone()))
                            as Arc<dyn nero_processor::torrent::TorrentBackend>
                    });

                    let processor = Processor::with_config(
                        processor_addr,
                        http_client,
                        nero_processor::Config {
                            torrent_backend,
                            ..Default::default()
                        },
                    );

                    let nero = ExtensionHost::new(processor);
                    let state = PluginState::new(nero);

                    let processor = state.host.processor().clone();
                    tauri::async_runtime::spawn(async move {
                        processor
                            .run()
                            .await
                            .expect("Unable to spawn internal extension processor server")
                    });

                    app_handle.manage(state);
                    app_handle.emit("nero-extensions://ready", ()).unwrap();
                });

                Ok(())
            })
            .invoke_handler(tauri::generate_handler![
                get_status,
                extensions::get_extension_metadata,
                extensions::load_extension,
                extensions::get_filters,
                extensions::search,
                extensions::get_series_info,
                extensions::get_series_episodes,
                extensions::get_series_videos,
            ])
            .build()
    }
}

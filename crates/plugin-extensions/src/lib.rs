mod extensions;
mod torrent;

use std::net::SocketAddr;

use libnero::{Nero, Processor};
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
    metadata: serde_json::Value,
    cache_dir: String,
    max_cache_size: Option<u64>,
}

#[derive(Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct PluginStatus {
    extension: Option<ExtensionInfo>,
    torrent_enabled: bool,
    torrent_output_folder: Option<String>,
}

struct PluginState {
    nero: Nero,
    http_client: Client,
    status: RwLock<PluginStatus>,
}

impl PluginState {
    fn new(nero: Nero, http_client: Client) -> Self {
        Self {
            nero,
            http_client,
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
        let http_client = Client::new();
        let nero = Nero::new(Processor::new(self.processor_addr, http_client.clone()));
        let state = PluginState::new(nero, http_client);

        plugin::Builder::new("nero-extensions")
            .setup(|app, _| {
                let processor = state.nero.processor().clone();
                tauri::async_runtime::spawn(async move {
                    processor
                        .run()
                        .await
                        .expect("Unable to spawn internal extension processor server")
                });
                app.manage(state);
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
                torrent::enable_torrent_support,
                torrent::disable_torrent_support,
            ])
            .build()
    }
}

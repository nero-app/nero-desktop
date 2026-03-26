use std::net::SocketAddr;

use libnero::{
    ExtensionMetadata, Nero, Processor,
    types::{EpisodesPage, FilterCategory, SearchFilter, Series, SeriesPage, Video},
};
use reqwest::Client;
use tauri::{
    Manager, Result, Runtime, State,
    plugin::{self, TauriPlugin},
};

struct PluginState {
    nero: Nero,
    http_client: Client,
}

#[tauri::command]
#[tracing::instrument]
async fn get_extension_metadata(file_path: String) -> Result<ExtensionMetadata> {
    Nero::get_extension_metadata(file_path)
        .await
        .map_err(Into::into)
}

#[tauri::command]
#[tracing::instrument(skip(state))]
async fn load_extension(state: State<'_, PluginState>, file_path: String) -> Result<()> {
    state
        .nero
        .load_extension(file_path)
        .await
        .map_err(Into::into)
}

#[tauri::command]
#[tracing::instrument(skip(state))]
async fn enable_torrent_support(
    state: State<'_, PluginState>,
    output_folder: String,
) -> Result<()> {
    state
        .nero
        .enable_torrent_support(output_folder.into(), state.http_client.clone())
        .await
        .map_err(Into::into)
}

#[tauri::command]
#[tracing::instrument(skip(state))]
async fn disable_torrent_support(state: State<'_, PluginState>) -> Result<()> {
    state
        .nero
        .disable_torrent_support()
        .await
        .map_err(Into::into)
}

#[tauri::command]
#[tracing::instrument(skip(state))]
async fn get_filters(state: State<'_, PluginState>) -> Result<Vec<FilterCategory>> {
    state.nero.get_filters().await.map_err(Into::into)
}

#[tauri::command]
#[tracing::instrument(skip(state))]
async fn search(
    state: State<'_, PluginState>,
    query: &str,
    page: Option<u16>,
    filters: Vec<SearchFilter>,
) -> Result<SeriesPage> {
    state
        .nero
        .search(query, page, filters)
        .await
        .map_err(Into::into)
}

#[tauri::command]
#[tracing::instrument(skip(state))]
async fn get_series_info(state: State<'_, PluginState>, series_id: &str) -> Result<Series> {
    state
        .nero
        .get_series_info(series_id)
        .await
        .map_err(Into::into)
}

#[tauri::command]
#[tracing::instrument(skip(state))]
async fn get_series_episodes(
    state: State<'_, PluginState>,
    series_id: &str,
    page: Option<u16>,
) -> Result<EpisodesPage> {
    state
        .nero
        .get_series_episodes(series_id, page)
        .await
        .map_err(Into::into)
}

#[tauri::command]
#[tracing::instrument(skip(state))]
async fn get_series_videos(
    state: State<'_, PluginState>,
    series_id: &str,
    episode_id: &str,
    episode_number: u32,
) -> Result<Vec<Video>> {
    state
        .nero
        .get_series_videos(series_id, episode_id, episode_number)
        .await
        .map_err(Into::into)
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
        let state = PluginState {
            nero: Nero::new(Processor::new(self.processor_addr, http_client.clone())),
            http_client,
        };

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
                get_extension_metadata,
                load_extension,
                enable_torrent_support,
                disable_torrent_support,
                get_filters,
                search,
                get_series_info,
                get_series_episodes,
                get_series_videos
            ])
            .build()
    }
}

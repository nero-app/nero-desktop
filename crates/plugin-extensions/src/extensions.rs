use libnero::{
    ExtensionHost, ExtensionMetadata,
    types::{
        EpisodesPage, ExtensionOptions, FilterCategory, SearchFilter, Series, SeriesPage, Video,
    },
};
use tauri::{AppHandle, Result, Runtime, State};

use crate::{ExtensionInfo, PluginState};

#[tauri::command]
#[tracing::instrument]
pub async fn get_extension_metadata(file_path: String) -> Result<ExtensionMetadata> {
    ExtensionHost::get_extension_metadata(file_path)
        .await
        .map_err(Into::into)
}

#[tauri::command]
#[tracing::instrument(skip(state, app))]
pub async fn load_extension<R: Runtime>(
    state: State<'_, PluginState>,
    app: AppHandle<R>,
    file_path: String,
    options: ExtensionOptions,
) -> Result<ExtensionInfo> {
    let cache_dir = options.cache_dir.to_string_lossy().to_string();
    let max_cache_size = options.max_cache_size;

    let extension = state
        .host
        .load(file_path.clone(), options)
        .await
        .map_err(|e| tauri::Error::Anyhow(e.into()))?;

    let info = ExtensionInfo {
        file_path,
        metadata: extension.metadata(),
        cache_dir,
        max_cache_size,
    };

    {
        state.extension.write().await.replace(extension);

        let mut status = state.status.write().await;
        status.extension = Some(info.clone());
    }

    state.emit_status(&app).await?;

    Ok(info)
}

#[tauri::command]
#[tracing::instrument(skip(state))]
pub async fn get_filters(state: State<'_, PluginState>) -> Result<Vec<FilterCategory>> {
    let guard = state.extension.read().await;
    let extension = guard
        .as_ref()
        .ok_or(anyhow::anyhow!("extension not loaded"))?;

    extension.get_filters().await.map_err(Into::into)
}

#[tauri::command]
#[tracing::instrument(skip(state))]
pub async fn search(
    state: State<'_, PluginState>,
    query: &str,
    page: Option<u16>,
    filters: Vec<SearchFilter>,
) -> Result<SeriesPage> {
    let guard = state.extension.read().await;
    let extension = guard
        .as_ref()
        .ok_or(anyhow::anyhow!("extension not loaded"))?;

    extension
        .search(query, page, filters)
        .await
        .map_err(Into::into)
}

#[tauri::command]
#[tracing::instrument(skip(state))]
pub async fn get_series_info(state: State<'_, PluginState>, series_id: &str) -> Result<Series> {
    let guard = state.extension.read().await;
    let extension = guard
        .as_ref()
        .ok_or(anyhow::anyhow!("extension not loaded"))?;

    extension
        .get_series_info(series_id)
        .await
        .map_err(Into::into)
}

#[tauri::command]
#[tracing::instrument(skip(state))]
pub async fn get_series_episodes(
    state: State<'_, PluginState>,
    series_id: &str,
    page: Option<u16>,
) -> Result<EpisodesPage> {
    let guard = state.extension.read().await;
    let extension = guard
        .as_ref()
        .ok_or(anyhow::anyhow!("extension not loaded"))?;

    extension
        .get_series_episodes(series_id, page)
        .await
        .map_err(Into::into)
}

#[tauri::command]
#[tracing::instrument(skip(state))]
pub async fn get_series_videos(
    state: State<'_, PluginState>,
    series_id: &str,
    episode_id: &str,
    episode_number: u32,
) -> Result<Vec<Video>> {
    let guard = state.extension.read().await;
    let extension = guard
        .as_ref()
        .ok_or(anyhow::anyhow!("extension not loaded"))?;

    extension
        .get_series_videos(series_id, episode_id, episode_number)
        .await
        .map_err(Into::into)
}

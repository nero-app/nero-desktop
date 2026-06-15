use std::sync::Arc;

use libnero::{
    ExtensionHost, ExtensionMetadata,
    types::{
        EpisodesPage, ExtensionOptions, FilterCategory, SearchFilter, Series, SeriesPage, Video,
    },
};
use serde::Serialize;
use tauri::{AppHandle, Result, Runtime, State, async_runtime::RwLock};
use uuid::Uuid;

use crate::{
    PluginState,
    preferences::{PreferencesData, StoredExtension},
    torrent_resolver::{ExtensionSearcher, TorrentResolver},
};

#[tauri::command]
#[tracing::instrument]
pub async fn get_extension_metadata(file_path: String) -> Result<ExtensionMetadata> {
    ExtensionHost::get_extension_metadata(file_path)
        .await
        .map_err(Into::into)
}

#[tauri::command]
#[tracing::instrument(skip(state, preferences, app))]
pub async fn load_extension<R: Runtime>(
    state: State<'_, PluginState>,
    preferences: State<'_, RwLock<PreferencesData>>,
    app: AppHandle<R>,
    file_path: String,
    options: ExtensionOptions,
) -> Result<()> {
    let cache_dir = options.cache_dir.to_string_lossy().to_string();
    let max_cache_size = options.max_cache_size;

    let extension = state.host.load(file_path.clone(), options).await?;
    let id = Uuid::new_v4();

    state
        .extensions
        .write()
        .await
        .insert(id, Arc::new(extension));

    let mut data = preferences.write().await;
    data.extensions.retain(|e| e.file_path != file_path);
    data.extensions.push(StoredExtension {
        id,
        file_path,
        options: crate::preferences::ExtensionOptions {
            cache_dir,
            max_cache_size,
        },
    });
    data.save(&app)?;

    Ok(())
}

#[tauri::command]
#[tracing::instrument(skip(state, preferences, app))]
pub async fn unload_extension<R: Runtime>(
    state: State<'_, PluginState>,
    preferences: State<'_, RwLock<PreferencesData>>,
    app: AppHandle<R>,
    extension_id: Uuid,
) -> Result<()> {
    state.extensions.write().await.remove(&extension_id);

    let mut data = preferences.write().await;
    data.extensions.retain(|e| e.id != extension_id);
    data.save(&app)?;

    Ok(())
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadedExtension {
    id: Uuid,
    file_path: String,
    options: crate::preferences::ExtensionOptions,
    metadata: Arc<ExtensionMetadata>,
}

#[tauri::command]
#[tracing::instrument(skip(state, preferences))]
pub async fn get_loaded_extensions(
    state: State<'_, PluginState>,
    preferences: State<'_, RwLock<PreferencesData>>,
) -> Result<Vec<LoadedExtension>> {
    let guard = state.extensions.read().await;
    let prefs = preferences.read().await;

    let extensions = guard
        .iter()
        .map(|(id, extension)| {
            let stored_extension = prefs
                .extensions
                .iter()
                .find(|e| e.id == *id)
                .cloned()
                .unwrap_or_default();
            LoadedExtension {
                id: stored_extension.id,
                file_path: stored_extension.file_path,
                options: stored_extension.options,
                metadata: extension.metadata().clone(),
            }
        })
        .collect();

    Ok(extensions)
}

#[tauri::command]
#[tracing::instrument(skip(state))]
pub async fn get_filters(
    state: State<'_, PluginState>,
    extension_id: Uuid,
) -> Result<Vec<FilterCategory>> {
    let guard = state.extensions.read().await;
    let extension = guard
        .get(&extension_id)
        .ok_or_else(|| anyhow::anyhow!("extension not loaded: {extension_id}"))?;

    extension.get_filters().await.map_err(Into::into)
}

#[tauri::command]
#[tracing::instrument(skip(state))]
pub async fn search(
    state: State<'_, PluginState>,
    extension_id: Uuid,
    query: &str,
    page: Option<u16>,
    filters: Vec<SearchFilter>,
) -> Result<SeriesPage> {
    let guard = state.extensions.read().await;
    let extension = guard
        .get(&extension_id)
        .ok_or_else(|| anyhow::anyhow!("extension not loaded: {extension_id}"))?;

    extension
        .search(query, page, filters)
        .await
        .map_err(Into::into)
}

#[tauri::command]
#[tracing::instrument(skip(state))]
pub async fn get_series_info(
    state: State<'_, PluginState>,
    extension_id: Uuid,
    series_id: &str,
) -> Result<Series> {
    let guard = state.extensions.read().await;
    let extension = guard
        .get(&extension_id)
        .ok_or_else(|| anyhow::anyhow!("extension not loaded: {extension_id}"))?;

    extension
        .get_series_info(series_id)
        .await
        .map_err(Into::into)
}

#[tauri::command]
#[tracing::instrument(skip(state))]
pub async fn get_series_episodes(
    state: State<'_, PluginState>,
    extension_id: Uuid,
    series_id: &str,
    page: Option<u16>,
) -> Result<EpisodesPage> {
    let guard = state.extensions.read().await;
    let extension = guard
        .get(&extension_id)
        .ok_or_else(|| anyhow::anyhow!("extension not loaded: {extension_id}"))?;

    extension
        .get_series_episodes(series_id, page)
        .await
        .map_err(Into::into)
}

#[tauri::command]
#[tracing::instrument(skip(state))]
pub async fn get_series_videos(
    state: State<'_, PluginState>,
    extension_id: Uuid,
    series_id: &str,
    episode_id: &str,
    episode_number: u32,
) -> Result<Vec<Video>> {
    let guard = state.extensions.read().await;
    let extension = guard
        .get(&extension_id)
        .ok_or_else(|| anyhow::anyhow!("extension not loaded: {extension_id}"))?;

    if let Some(handle) = &state.torrent_resolver_handle {
        let resolver = Arc::new(TorrentResolver {
            searcher: ExtensionSearcher(extension.clone()),
            series_id: series_id.to_string(),
            episode_number,
        });
        handle.set(resolver).await;
    }

    extension
        .get_series_videos(series_id, episode_id)
        .await
        .map_err(Into::into)
}

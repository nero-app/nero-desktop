use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Result, Runtime, State, async_runtime::RwLock};
use tauri_plugin_store::StoreExt;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MediaProxyPreferences {
    pub torrent_enabled: bool,
    pub torrent_output_folder: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionOptions {
    pub cache_dir: String,
    pub max_cache_size: Option<u64>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StoredExtension {
    pub id: Uuid,
    pub file_path: String,
    pub options: ExtensionOptions,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PreferencesData {
    pub extensions: Vec<StoredExtension>,
    pub media_proxy: Option<MediaProxyPreferences>,
}

impl PreferencesData {
    const STORE_FILE: &'static str = "nero-extensions.json";
    const PREFERENCES_KEY: &'static str = "preferences";

    pub fn new<R: Runtime>(app: &AppHandle<R>) -> Self {
        app.store(Self::STORE_FILE)
            .ok()
            .and_then(|store| store.get(Self::PREFERENCES_KEY))
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default()
    }

    pub fn save<R: Runtime>(&self, app: &AppHandle<R>) -> tauri::Result<()> {
        let store = app.store(Self::STORE_FILE).unwrap();
        store.set(Self::PREFERENCES_KEY, serde_json::to_value(self).unwrap());
        store.save().unwrap();
        Ok(())
    }
}

#[tauri::command]
pub async fn get_media_proxy_preferences(
    preferences: State<'_, RwLock<PreferencesData>>,
) -> Result<Option<MediaProxyPreferences>> {
    Ok(preferences.read().await.media_proxy.clone())
}

#[tauri::command]
pub async fn set_media_proxy_preferences<R: Runtime>(
    preferences: tauri::State<'_, RwLock<PreferencesData>>,
    app: AppHandle<R>,
    media_proxy: MediaProxyPreferences,
) -> Result<()> {
    let mut data = preferences.write().await;
    data.media_proxy = Some(media_proxy.clone());
    data.save(&app)?;
    Ok(())
}

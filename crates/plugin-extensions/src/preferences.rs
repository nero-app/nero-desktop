use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Result, Runtime, State, async_runtime::RwLock};
use tauri_plugin_store::StoreExt;

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorPreferences {
    pub torrent_enabled: bool,
    pub torrent_output_folder: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionPreferences {
    pub file_path: String,
    pub cache_dir: String,
    pub max_cache_size: Option<u64>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PreferencesData {
    pub extension: Option<ExtensionPreferences>,
    pub processor: Option<ProcessorPreferences>,
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
        app.emit("nero-extensions://preferences-changed", self)?;
        Ok(())
    }
}

#[tauri::command]
pub async fn get_preferences(
    preferences: State<'_, RwLock<PreferencesData>>,
) -> Result<PreferencesData> {
    Ok(preferences.read().await.clone())
}

#[tauri::command]
pub async fn set_processor_preferences<R: Runtime>(
    preferences: tauri::State<'_, RwLock<PreferencesData>>,
    app: AppHandle<R>,
    processor: ProcessorPreferences,
) -> Result<()> {
    let mut data = preferences.write().await;
    data.processor = Some(processor);
    data.save(&app)
}

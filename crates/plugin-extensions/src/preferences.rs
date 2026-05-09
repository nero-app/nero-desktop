use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Result, Runtime};
use tauri_plugin_store::StoreExt;

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorPreferences {
    pub torrent_enabled: bool,
    pub torrent_output_folder: Option<String>,
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

pub struct PluginPreferences;

impl PluginPreferences {
    const STORE_FILE: &'static str = "nero.json";
    const PREFERENCES_KEY: &'static str = "preferences";

    pub fn get<R: Runtime>(app: &AppHandle<R>) -> PreferencesData {
        app.store(Self::STORE_FILE)
            .ok()
            .and_then(|store| store.get(Self::PREFERENCES_KEY))
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default()
    }

    pub fn save<R: Runtime>(app: &AppHandle<R>, data: &PreferencesData) -> tauri::Result<()> {
        let store = app.store(Self::STORE_FILE).unwrap();
        store.set(Self::PREFERENCES_KEY, serde_json::to_value(data).unwrap());
        store.save().unwrap();
        app.emit("nero-extensions://preferences-changed", data)?;
        Ok(())
    }
}

#[tauri::command]
pub fn get_preferences<R: Runtime>(app: AppHandle<R>) -> PreferencesData {
    PluginPreferences::get(&app)
}

#[tauri::command]
pub async fn set_processor_preferences<R: Runtime>(
    app: AppHandle<R>,
    processor: ProcessorPreferences,
) -> Result<()> {
    let mut data = PluginPreferences::get(&app);
    data.processor = Some(processor);
    PluginPreferences::save(&app, &data)?;
    Ok(())
}

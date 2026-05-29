use serde::{Deserialize, Serialize};
use tauri::{async_runtime::RwLock, AppHandle, Result, Runtime, State};
use tauri_plugin_store::StoreExt;

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PreferencesData {
    pub player_path: Option<String>,
}

impl PreferencesData {
    const STORE_FILE: &'static str = "app.json";
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
pub async fn get_preferences(
    preferences: State<'_, RwLock<PreferencesData>>,
) -> Result<PreferencesData> {
    Ok(preferences.read().await.clone())
}

#[tauri::command]
pub async fn set_preferences<R: Runtime>(
    preferences: tauri::State<'_, RwLock<PreferencesData>>,
    app: AppHandle<R>,
    data: PreferencesData,
) -> Result<()> {
    data.save(&app)?;
    *preferences.write().await = data;
    Ok(())
}

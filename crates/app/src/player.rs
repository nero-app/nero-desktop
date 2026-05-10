use std::process::Command;
use tauri::{async_runtime::RwLock, Result, State};

use crate::{preferences::PreferencesData, AppState};

#[tauri::command]
#[tracing::instrument(skip(state, preferences))]
pub async fn open_video_player(
    state: State<'_, AppState>,
    preferences: State<'_, RwLock<PreferencesData>>,
    url: String,
) -> Result<()> {
    let player_path = preferences
        .read()
        .await
        .player_path
        .clone()
        .ok_or_else(|| tauri::Error::Anyhow(anyhow::anyhow!("No player configured")))?;

    let mut guard = state
        .player_process
        .lock()
        .map_err(|_| anyhow::anyhow!("Mutex poisoned"))?;

    if let Some(mut child) = guard.take() {
        let _ = child.kill();
    }

    #[cfg(target_os = "macos")]
    let exec_path = if player_path.ends_with(".app") {
        let app_name = player_path
            .trim_end_matches(".app")
            .split('/')
            .next_back()
            .unwrap_or("");
        format!("{}/Contents/MacOS/{}", player_path, app_name)
    } else {
        player_path
    };

    #[cfg(not(target_os = "macos"))]
    let exec_path = player_path;

    let child = Command::new(exec_path)
        .arg(&url)
        .spawn()
        .map_err(|e| anyhow::anyhow!("Failed to start player: {}", e))?;

    *guard = Some(child);

    Ok(())
}

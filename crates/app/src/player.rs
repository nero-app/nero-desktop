use std::process::Command;
use tauri::{AppHandle, Result, State};

use crate::AppState;

#[tauri::command]
pub fn set_player_path(path: String, state: State<AppState>, app: AppHandle) -> Result<()> {
    {
        let mut status = state
            .status
            .lock()
            .map_err(|_| anyhow::anyhow!("Mutex poisoned"))?;
        status.player_path = Some(path);
    }
    state.emit_status(&app)
}

#[tauri::command]
#[tracing::instrument(skip(state))]
pub fn open_video_player(state: State<AppState>, url: String) -> Result<()> {
    let player_path = {
        let status = state
            .status
            .lock()
            .map_err(|_| anyhow::anyhow!("Mutex poisoned"))?;
        status
            .player_path
            .clone()
            .ok_or_else(|| tauri::Error::Anyhow(anyhow::anyhow!("No player configured")))?
    };

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

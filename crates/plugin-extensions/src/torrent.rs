use tauri::{AppHandle, Result, Runtime, State};

use crate::PluginState;

#[tauri::command]
#[tracing::instrument(skip(state, app))]
pub async fn enable_torrent_support<R: Runtime>(
    state: State<'_, PluginState>,
    app: AppHandle<R>,
    output_folder: String,
) -> Result<()> {
    state
        .nero
        .enable_torrent_support(output_folder.clone().into(), state.http_client.clone())
        .await
        .map_err(|e| tauri::Error::Anyhow(e.into()))?;

    {
        let mut status = state.status.write().await;
        status.torrent_enabled = true;
        status.torrent_output_folder = Some(output_folder);
    }

    state.emit_status(&app).await
}

#[tauri::command]
#[tracing::instrument(skip(state, app))]
pub async fn disable_torrent_support<R: Runtime>(
    state: State<'_, PluginState>,
    app: AppHandle<R>,
) -> Result<()> {
    state
        .nero
        .disable_torrent_support()
        .await
        .map_err(|e| tauri::Error::Anyhow(e.into()))?;

    {
        let mut status = state.status.write().await;
        status.torrent_enabled = false;
        status.torrent_output_folder = None;
    }

    state.emit_status(&app).await
}

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod player;

use std::{net::SocketAddr, process::Child, sync::Mutex};

use serde::Serialize;
use tauri::{AppHandle, Emitter, Result, State};

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct AppStatus {
    player_path: Option<String>,
}

struct AppState {
    status: Mutex<AppStatus>,
    player_process: Mutex<Option<Child>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            status: Mutex::new(AppStatus { player_path: None }),
            player_process: Mutex::new(None),
        }
    }

    fn emit_status(&self, app: &AppHandle) -> Result<()> {
        let status = self.status.lock().unwrap().clone();
        app.emit("app://status-changed", status)
    }
}

#[tauri::command]
fn get_status(state: State<AppState>) -> AppStatus {
    state.status.lock().unwrap().clone()
}

fn main() {
    tracing_subscriber::fmt().init();

    let processor_port = portpicker::pick_unused_port()
        .expect("failed to pick an unused port for the extensions processor");
    let processor_addr = SocketAddr::from(([127, 0, 0, 1], processor_port));

    tauri::Builder::default()
        .manage(AppState::new())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_nero_extensions::Builder::new(processor_addr).build())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            get_status,
            player::set_player_path,
            player::open_video_player,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

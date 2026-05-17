#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod player;
mod preferences;

use std::{net::SocketAddr, process::Child, sync::Mutex};

use tauri::{async_runtime::RwLock, Manager};

use crate::preferences::PreferencesData;

struct AppState {
    player_process: Mutex<Option<Child>>,
}

fn main() {
    tracing_subscriber::fmt().init();

    let proxy_port = portpicker::pick_unused_port()
        .expect("failed to pick an unused port for the extensions media proxy");
    let proxy_addr = SocketAddr::from(([127, 0, 0, 1], proxy_port));

    tauri::Builder::default()
        .manage(AppState {
            player_process: Mutex::new(None),
        })
        .setup(|app| {
            app.manage(RwLock::new(PreferencesData::new(app.handle())));
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_nero_extensions::init(proxy_addr))
        .invoke_handler(tauri::generate_handler![
            preferences::get_preferences,
            preferences::set_preferences,
            player::open_video_player,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

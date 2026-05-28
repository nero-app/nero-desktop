const COMMANDS: &[&str] = &[
    "get_media_proxy_preferences",
    "set_media_proxy_preferences",
    "get_extension_metadata",
    "load_extension",
    "unload_extension",
    "get_loaded_extensions",
    "get_filters",
    "search",
    "get_series_info",
    "get_series_episodes",
    "get_series_videos",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}

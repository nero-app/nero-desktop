const COMMANDS: &[&str] = &[
    "get_preferences",
    "set_media_proxy_preferences",
    "get_extension_metadata",
    "load_extension",
    "get_filters",
    "search",
    "get_series_info",
    "get_series_episodes",
    "get_series_videos",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}

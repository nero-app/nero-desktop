mod extensions;
mod preferences;

use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use libnero::{Extension, ExtensionHost, types::ExtensionOptions};
use librqbit::Session;
use nero_processor::{
    Processor,
    torrent::{RqbitTorrentBackend, TorrentBackend},
};
use reqwest::Client;
use tauri::{
    Manager, Runtime,
    plugin::{self, TauriPlugin},
};

use tauri::{Emitter, Result, async_runtime::RwLock};

use crate::preferences::PreferencesData;

struct PluginState {
    host: ExtensionHost,
    extension: RwLock<Option<Extension>>,
}

impl PluginState {
    async fn new(data: &PreferencesData, processor_addr: SocketAddr) -> Result<Self> {
        let http_client = Client::new();

        let torrent_backend = if let Some(torrent) = data.processor.as_ref()
            && torrent.torrent_enabled
        {
            let output_dir = PathBuf::from(&torrent.torrent_output_folder);
            let librqbit_session = Session::new(output_dir).await?;
            let backend = RqbitTorrentBackend::new(librqbit_session, http_client.clone());
            Some(Arc::new(backend) as Arc<dyn TorrentBackend + 'static>)
        } else {
            None
        };

        let processor = Processor::with_config(
            processor_addr,
            http_client,
            nero_processor::Config {
                torrent_backend,
                ..Default::default()
            },
        );

        let host = ExtensionHost::new(processor);

        let initial_extension = if let Some(prefs) = data.extension.as_ref() {
            let options = ExtensionOptions {
                cache_dir: PathBuf::from(&prefs.cache_dir),
                max_cache_size: prefs.max_cache_size,
            };
            let extension = host.load(&prefs.file_path, options).await?;
            Some(extension)
        } else {
            None
        };

        Ok(Self {
            host,
            extension: RwLock::new(initial_extension),
        })
    }
}

pub fn init<R: Runtime>(processor_addr: SocketAddr) -> TauriPlugin<R> {
    plugin::Builder::new("nero-extensions")
        .setup(move |app, _| {
            let preferences = PreferencesData::new(app);
            let app_handle = app.clone();
            tauri::async_runtime::spawn(async move {
                let state = PluginState::new(&preferences, processor_addr)
                    .await
                    .unwrap();

                let processor = state.host.processor().clone();
                tauri::async_runtime::spawn(async move { processor.run().await.unwrap() });

                app_handle.manage(RwLock::new(preferences));
                app_handle.manage(state);
                app_handle.emit("nero-extensions://ready", ()).unwrap();
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            preferences::get_preferences,
            preferences::set_processor_preferences,
            extensions::get_extension_metadata,
            extensions::load_extension,
            extensions::get_filters,
            extensions::search,
            extensions::get_series_info,
            extensions::get_series_episodes,
            extensions::get_series_videos,
        ])
        .build()
}

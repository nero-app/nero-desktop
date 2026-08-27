use std::path::{Path, PathBuf};

use crate::i18n::Language;

#[derive(Debug, Clone)]
pub enum PreferenceAction {
    Language(Language),
    PlayerPath(PathBuf),
    TorrentEnabled(bool),
    TorrentOutputFolder(Option<PathBuf>),
}

#[derive(Debug, Clone, Default)]
pub struct Preferences {
    language: Language,
    player_path: Option<PathBuf>,
    media_proxy: MediaProxyPreferences,
}

impl Preferences {
    pub fn player_path(&self) -> Option<&Path> {
        self.player_path.as_deref()
    }

    pub fn language(&self) -> Language {
        self.language
    }

    pub fn media_proxy(&self) -> &MediaProxyPreferences {
        &self.media_proxy
    }

    pub fn update(&mut self, action: PreferenceAction) -> Option<Language> {
        match action {
            PreferenceAction::Language(language) => {
                self.language = language;
                Some(language)
            }
            PreferenceAction::PlayerPath(path) => {
                self.player_path = Some(path);
                None
            }
            PreferenceAction::TorrentEnabled(enabled) => {
                self.media_proxy.torrent_enabled = enabled;
                None
            }
            PreferenceAction::TorrentOutputFolder(folder) => {
                self.media_proxy.torrent_output_folder = folder;
                None
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct MediaProxyPreferences {
    pub torrent_enabled: bool,
    pub torrent_output_folder: Option<PathBuf>,
}

impl Default for MediaProxyPreferences {
    fn default() -> Self {
        Self {
            torrent_enabled: true,
            torrent_output_folder: None,
        }
    }
}

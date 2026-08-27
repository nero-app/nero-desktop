use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    Server(String),
    Url(String),
    BackgroundTask {
        operation: &'static str,
        detail: String,
    },
    Storage {
        operation: &'static str,
        detail: String,
    },
    Http {
        operation: &'static str,
        detail: String,
    },
    Extension {
        operation: &'static str,
        detail: String,
    },
    MissingExtensionName,
    EmptyExtensionName,
    MissingExtensionVersion,
    EmptyExtensionVersion,
    ExtensionAlreadyLoaded(String),
    Torrent(String),
    TorrentDisabled,
    UnsupportedImageResource,
    PlayerLockPoisoned,
    PlayerSpawn(String),
}

impl Error {
    pub fn task(operation: &'static str, error: impl fmt::Display) -> Self {
        Self::BackgroundTask {
            operation,
            detail: error.to_string(),
        }
    }

    pub fn storage(operation: &'static str, error: impl fmt::Display) -> Self {
        Self::Storage {
            operation,
            detail: error.to_string(),
        }
    }

    pub fn http(operation: &'static str, error: impl fmt::Display) -> Self {
        Self::Http {
            operation,
            detail: error.to_string(),
        }
    }

    pub fn extension(operation: &'static str, error: impl fmt::Display) -> Self {
        Self::Extension {
            operation,
            detail: error.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Server(detail) => write!(formatter, "application server failed: {detail}"),
            Self::Url(detail) => write!(formatter, "invalid URL: {detail}"),
            Self::BackgroundTask { operation, detail } => {
                write!(formatter, "{operation} task failed: {detail}")
            }
            Self::Storage { operation, detail } => {
                write!(formatter, "failed to {operation}: {detail}")
            }
            Self::Http { operation, detail } => {
                write!(formatter, "HTTP request failed while {operation}: {detail}")
            }
            Self::Extension { operation, detail } => {
                write!(formatter, "extension failed to {operation}: {detail}")
            }
            Self::MissingExtensionName => {
                formatter.write_str("extension component metadata is missing `name`")
            }
            Self::EmptyExtensionName => {
                formatter.write_str("extension component metadata `name` cannot be empty")
            }
            Self::MissingExtensionVersion => {
                formatter.write_str("extension component metadata is missing `version`")
            }
            Self::EmptyExtensionVersion => {
                formatter.write_str("extension component metadata `version` cannot be empty")
            }
            Self::ExtensionAlreadyLoaded(id) => write!(formatter, "extension already loaded: {id}"),
            Self::Torrent(detail) => write!(formatter, "torrent session failed: {detail}"),
            Self::TorrentDisabled => formatter.write_str("torrent playback is disabled"),
            Self::UnsupportedImageResource => {
                formatter.write_str("an image resource must be an HTTP request")
            }
            Self::PlayerLockPoisoned => formatter.write_str("the player lock was poisoned"),
            Self::PlayerSpawn(detail) => {
                write!(formatter, "failed to start video player: {detail}")
            }
        }
    }
}

impl std::error::Error for Error {}

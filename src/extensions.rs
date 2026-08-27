mod keyvalue;
mod portal;
mod registry;

use std::fmt;
use std::path::PathBuf;

use crate::error::{Error, Result};

pub use self::registry::{LoadedExtension, Registry};
pub use nero_extensions::Metadata as ExtensionMetadata;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExtensionId(String);

impl TryFrom<&ExtensionMetadata> for ExtensionId {
    type Error = Error;

    fn try_from(metadata: &ExtensionMetadata) -> Result<Self> {
        let name = metadata
            .name
            .as_deref()
            .ok_or(Error::MissingExtensionName)?;

        if name.trim().is_empty() {
            return Err(Error::EmptyExtensionName);
        }

        let version = metadata
            .version
            .as_ref()
            .ok_or(Error::MissingExtensionVersion)?
            .to_string();

        if version.trim().is_empty() {
            return Err(Error::EmptyExtensionVersion);
        }

        Ok(Self(format!("{name}@{version}")))
    }
}

impl AsRef<str> for ExtensionId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ExtensionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Options {
    pub cache_dir: PathBuf,
    pub max_cache_size: Option<CacheLimit>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            cache_dir: default_cache_dir(),
            max_cache_size: Some(CacheLimit::from_megabytes(128)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CacheLimit(u64);

impl CacheLimit {
    pub const MAX: Self = Self::from_megabytes(512);

    pub const fn from_megabytes(megabytes: u64) -> Self {
        Self(megabytes.saturating_mul(1024 * 1024))
    }

    pub const fn megabytes(self) -> u64 {
        self.0 / (1024 * 1024)
    }

    pub const fn bytes(self) -> u64 {
        self.0
    }
}

impl fmt::Display for CacheLimit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} MB", self.megabytes())
    }
}

pub(super) fn cache_key(id: &ExtensionId) -> String {
    url::form_urlencoded::byte_serialize(id.as_ref().as_bytes()).collect()
}

pub fn default_cache_dir() -> PathBuf {
    dirs::cache_dir()
        .unwrap_or_else(std::env::temp_dir)
        .join("moe.nero.app")
}

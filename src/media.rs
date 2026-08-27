mod proxy;
mod torrent;

pub use self::proxy::{default_torrent_dir, Media};
pub use self::torrent::{ExtensionSearcher, TorrentResolver};

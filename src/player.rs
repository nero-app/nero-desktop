use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use mime::Mime;
use nero_extensions::types::MediaResource;
use nero_extensions::WasmExtension;
use nero_media_proxy::resources::Resource;
use nero_media_proxy::torrent::TorrentSource;
use reqwest::Client;
use url::Url;

use crate::error::{Error, Result};
use crate::media::{ExtensionSearcher, Media, TorrentResolver};

#[derive(Clone)]
pub struct Playback {
    player: Arc<Player>,
    client: Client,
    media: Media,
    proxy_base: Url,
}

impl Playback {
    pub fn new(client: Client, media: Media, proxy_base: Url) -> Self {
        Self {
            player: Arc::new(Player::default()),
            client,
            media,
            proxy_base,
        }
    }

    pub async fn play(
        &self,
        player_path: PathBuf,
        media: MediaResource,
        extension: Arc<WasmExtension>,
        series_id: String,
        episode_number: u32,
    ) -> Result<()> {
        let resource = match media {
            MediaResource::MagnetUri(uri) => Resource::Torrent(TorrentSource::MagnetUri(uri)),
            MediaResource::HttpRequest(request) => {
                let mime = mime_type(&self.client, &request).await?;

                if mime.is_some_and(|mime| mime.essence_str() == "application/x-bittorrent") {
                    Resource::Torrent(TorrentSource::Http(request))
                } else {
                    Resource::Http(request)
                }
            }
        };

        let resolver = matches!(&resource, Resource::Torrent(_)).then(|| TorrentResolver {
            searcher: ExtensionSearcher(extension),
            series_id,
            episode_number,
        });

        let reference = self.media.register_playback(resource, resolver).await?;
        let url = reference
            .resolve(&self.proxy_base)
            .map_err(|error| Error::Url(error.to_string()))?;
        let player = self.player.clone();

        tokio::task::spawn_blocking(move || player.play(&player_path, url.as_str()))
            .await
            .map_err(|error| Error::task("start video player", error))??;

        Ok(())
    }
}

#[derive(Default)]
pub struct Player {
    current: Mutex<Option<Child>>,
}

impl Player {
    fn play(&self, player_path: &Path, url: &str) -> Result<()> {
        let mut current = self.current.lock().map_err(|_| Error::PlayerLockPoisoned)?;

        if let Some(mut child) = current.take() {
            let _ = child.kill();
        }

        *current = Some(
            Command::new(executable(player_path))
                .arg(url)
                .spawn()
                .map_err(|error| Error::PlayerSpawn(error.to_string()))?,
        );

        Ok(())
    }
}

async fn mime_type(
    client: &Client,
    request: &nero_extensions::types::HttpRequest,
) -> Result<Option<Mime>> {
    if let Some(mime) = mime_guess::from_path(request.uri().path()).first() {
        return Ok(Some(mime));
    }

    let response = client
        .head(request.uri().to_string())
        .headers(request.headers().clone())
        .send()
        .await
        .map_err(|error| Error::http("probing media type", error))?;

    if response.status().is_success() {
        let mime = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| Mime::from_str(value).ok());

        if mime.is_some() {
            return Ok(mime);
        }
    }

    let mut request_builder = client
        .request(request.method().clone(), request.uri().to_string())
        .headers(request.headers().clone());

    if let Some(body) = request.body() {
        request_builder = request_builder.body(body.clone());
    }

    let mut response = request_builder
        .send()
        .await
        .map_err(|error| Error::http("probing media type", error))?;

    if !response.status().is_success() {
        return Ok(None);
    }

    let Some(chunk) = response
        .chunk()
        .await
        .map_err(|error| Error::http("reading media probe", error))?
    else {
        return Ok(None);
    };

    Ok(infer::get(&chunk).and_then(|kind| Mime::from_str(kind.mime_type()).ok()))
}

#[cfg(target_os = "macos")]
fn executable(player_path: &Path) -> PathBuf {
    let is_bundle = player_path
        .extension()
        .is_some_and(|extension| extension == "app");

    match (is_bundle, player_path.file_stem()) {
        (true, Some(name)) => player_path.join("Contents/MacOS").join(name),
        _ => player_path.to_path_buf(),
    }
}

#[cfg(not(target_os = "macos"))]
fn executable(player_path: &Path) -> PathBuf {
    player_path.to_path_buf()
}

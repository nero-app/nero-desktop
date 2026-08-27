use std::sync::Arc;

use anifile::{IndexedFile, SearchPage, Searcher};
use anyhow::Result;
use nero_extensions::types::Series;
use nero_extensions::{Extension, WasmExtension};
use nero_media_proxy::torrent::{TorrentFile, TorrentFileSelector};
use tokio::sync::RwLock;

pub struct SeriesWrapper(Series);

impl anifile::Series for SeriesWrapper {
    fn id(&self) -> &str {
        &self.0.id
    }
}

pub struct ExtensionSearcher(pub Arc<WasmExtension>);

impl Searcher for ExtensionSearcher {
    type Series = SeriesWrapper;

    async fn search(&self, query: &str, page: Option<u16>) -> Result<SearchPage<Self::Series>> {
        let result = self.0.search(query, page, vec![]).await?;

        Ok(SearchPage {
            items: result.items.into_iter().map(SeriesWrapper).collect(),
            has_next_page: result.has_next_page,
        })
    }
}

struct TorrentFileWrapper<'a>(&'a TorrentFile);

impl IndexedFile for TorrentFileWrapper<'_> {
    fn index(&self) -> usize {
        self.0.index
    }

    fn file_name(&self) -> Option<&str> {
        self.0.path.file_name()?.to_str()
    }
}

pub struct TorrentResolver {
    pub searcher: ExtensionSearcher,
    pub series_id: String,
    pub episode_number: u32,
}

#[async_trait::async_trait]
impl TorrentFileSelector for TorrentResolver {
    async fn select(&self, files: &[TorrentFile]) -> Result<Vec<usize>> {
        if files.len() == 1 {
            return Ok(vec![files[0].index]);
        }

        let files = files.iter().map(TorrentFileWrapper).collect::<Vec<_>>();
        let file_index =
            anifile::find_episode(&files, &self.searcher, &self.series_id, self.episode_number)
                .await?;

        match file_index {
            Some(index) => Ok(vec![index]),
            None => anyhow::bail!("no matching file found"),
        }
    }
}

#[derive(Default)]
pub struct TorrentResolverHandle {
    inner: RwLock<Option<Arc<TorrentResolver>>>,
}

impl TorrentResolverHandle {
    pub async fn set(&self, resolver: Arc<TorrentResolver>) {
        *self.inner.write().await = Some(resolver);
    }
}

#[async_trait::async_trait]
impl TorrentFileSelector for TorrentResolverHandle {
    async fn select(&self, files: &[TorrentFile]) -> Result<Vec<usize>> {
        let resolver = self
            .inner
            .read()
            .await
            .clone()
            .ok_or_else(|| anyhow::anyhow!("no torrent resolver configured"))?;

        resolver.select(files).await
    }
}

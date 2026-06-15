use anifile::{IndexedFile, SearchPage, Searcher};
use anyhow::Result;
use libnero::Extension;
use nero_media_proxy::torrent::{TorrentFile, TorrentFileSelector};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct SeriesWrapper(libnero::types::Series);

impl anifile::Series for SeriesWrapper {
    fn id(&self) -> &str {
        &self.0.id
    }
}

pub struct ExtensionSearcher(pub Arc<Extension>);

impl Searcher for ExtensionSearcher {
    type Series = SeriesWrapper;

    async fn search(
        &self,
        query: &str,
        page: Option<u16>,
    ) -> anyhow::Result<SearchPage<Self::Series>> {
        let result = self.0.search(query, page, vec![]).await?;
        Ok(SearchPage {
            items: result.items.into_iter().map(SeriesWrapper).collect(),
            has_next_page: result.has_next_page,
        })
    }
}

struct TorrentFileWrapper<'a>(&'a TorrentFile);

impl<'a> IndexedFile for TorrentFileWrapper<'a> {
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

        if let Some(index) = file_index {
            Ok(vec![index])
        } else {
            anyhow::bail!("no matching file found");
        }
    }
}

pub struct TorrentResolverHandle {
    inner: RwLock<Option<Arc<TorrentResolver>>>,
}

impl TorrentResolverHandle {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(None),
        }
    }

    pub async fn set(&self, resolver: Arc<TorrentResolver>) {
        *self.inner.write().await = Some(resolver);
    }
}

#[async_trait::async_trait]
impl TorrentFileSelector for TorrentResolverHandle {
    async fn select(&self, files: &[TorrentFile]) -> Result<Vec<usize>> {
        let guard = self.inner.read().await;
        match guard.as_ref() {
            Some(resolver) => resolver.select(files).await,
            None => anyhow::bail!("no torrent resolver configured"),
        }
    }
}

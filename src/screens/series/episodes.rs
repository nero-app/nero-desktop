use std::path::{Path, PathBuf};

use iced::task::Handle;
use iced::widget::{column, container, grid, sensor, space, text};
use iced::{Element, Fill, Task};
use nero_extensions::types::{Episode, EpisodesPage, MediaResource, Video};
use nero_extensions::Extension;
use rust_i18n::t;

use crate::components::dialog::overlay;
use crate::components::typography::TextExt;
use crate::error::{Error, Result};
use crate::extensions::LoadedExtension;
use crate::fetch::Fetch;
use crate::images::Images;
use crate::pagination::{Paginated, LOAD_MORE_MARGIN};
use crate::player::Playback;
use crate::screens::Action;
use crate::theme::PALETTE;
use crate::widgets::card::{episode_card, EPISODE_CARD_RATIO, EPISODE_GRID_SPACING};
use crate::widgets::video_selector;

enum Dialog {
    None,
    Videos {
        episode: Episode,
        videos: Fetch<Vec<Video>>,
        loading: Option<Handle>,
    },
}

pub struct Episodes {
    extension: LoadedExtension,
    series_id: String,
    playback: Playback,
    images: Images,

    episodes: Paginated<EpisodesPage>,
    dialog: Dialog,
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<EpisodesPage>),
    ImagesLoaded,
    LoadMore,
    Selected(Episode),
    VideosLoaded(Result<Vec<Video>>),
    VideoSelected(MediaResource),
    PlayerStarted(Result<()>),
    CloseDialog,
}

impl Episodes {
    pub fn new(
        playback: Playback,
        images: Images,
        extension: LoadedExtension,
        series_id: String,
    ) -> (Self, Task<Message>) {
        let episodes = Self {
            extension,
            series_id,
            playback,
            images,

            episodes: Paginated::default(),
            dialog: Dialog::None,
        };

        let load = episodes.load(1);

        (episodes, load)
    }

    pub fn update(&mut self, message: Message, player_path: Option<&Path>) -> Action<Message> {
        match message {
            Message::Loaded(result) => {
                let load_images = self.images.load_all(
                    result
                        .iter()
                        .flat_map(|page| page.items.iter())
                        .filter_map(|episode| episode.thumbnail_resource.clone()),
                    Message::ImagesLoaded,
                );
                self.episodes.loaded(result);
                Action::run(load_images)
            }
            Message::ImagesLoaded => Action::None,
            Message::LoadMore => {
                let Some(page) = self.episodes.start_loading_more() else {
                    return Action::None;
                };
                Action::run(self.load(page))
            }
            Message::Selected(episode) => self.open_videos(episode),
            Message::VideosLoaded(result) => {
                if let Dialog::Videos {
                    videos, loading, ..
                } = &mut self.dialog
                {
                    *loading = None;
                    *videos = result.into();
                }

                Action::None
            }
            Message::VideoSelected(media) => {
                let Dialog::Videos { episode, .. } = &self.dialog else {
                    return Action::None;
                };
                let Some(player_path) = player_path else {
                    return Action::None;
                };
                let play = self.play(media, u32::from(episode.number), player_path.to_path_buf());
                self.dialog = Dialog::None;

                Action::run(play)
            }
            Message::PlayerStarted(Ok(())) => Action::None,
            Message::PlayerStarted(Err(error)) => {
                tracing::error!(%error, "failed to start the video player");
                Action::None
            }
            Message::CloseDialog => {
                self.dialog = Dialog::None;
                Action::None
            }
        }
    }

    pub fn play_first(&mut self) -> Action<Message> {
        let first = self.episodes.items().next().cloned();

        match first {
            Some(episode) => self.open_videos(episode),
            None => Action::None,
        }
    }

    fn open_videos(&mut self, episode: Episode) -> Action<Message> {
        let (load, handle) = self.load_videos(&episode.id);

        self.dialog = Dialog::Videos {
            episode,
            videos: Fetch::Loading,
            loading: Some(handle),
        };

        Action::run(load)
    }

    pub fn overlay(&self, has_player: bool) -> Option<Element<'_, Message>> {
        match &self.dialog {
            Dialog::None => None,
            Dialog::Videos {
                episode, videos, ..
            } => Some(overlay(
                video_selector::dialog(
                    episode,
                    videos,
                    has_player,
                    &self.images,
                    Message::VideoSelected,
                    Message::CloseDialog,
                ),
                Message::CloseDialog,
            )),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let episodes = match self.episodes.data() {
            Fetch::Loading => return text(t!("common.loading")).body().into(),
            Fetch::Failed(error) => return text(error.to_string()).body().into(),
            Fetch::Loaded(episodes) => episodes,
        };

        let cards = grid(episodes.items.iter().map(|episode| {
            episode_card(
                episode,
                episode
                    .thumbnail_resource
                    .as_ref()
                    .and_then(|resource| self.images.handle(resource)),
                Message::Selected(episode.clone()),
            )
        }))
        .columns(3)
        .spacing(EPISODE_GRID_SPACING)
        .height(EPISODE_CARD_RATIO);

        let is_loading_more = self.episodes.is_loading_more();
        let item_count = self.episodes.items().count();
        let footer = (is_loading_more || self.episodes.can_load_more()).then(|| {
            let content = if is_loading_more {
                Element::from(
                    container(
                        text(t!("common.loading"))
                            .label()
                            .color(PALETTE.text_muted)
                            .center()
                            .width(Fill),
                    )
                    .width(Fill)
                    .padding([8, 0]),
                )
            } else {
                Element::from(space().height(1))
            };

            sensor(content)
                .key((item_count, is_loading_more))
                .anticipate(LOAD_MORE_MARGIN)
                .on_show(|_| Message::LoadMore)
        });

        column![cards, footer].spacing(8).into()
    }

    fn load(&self, page: u16) -> Task<Message> {
        let extension = self.extension.extension.clone();
        let series_id = self.series_id.clone();

        Task::perform(
            async move {
                extension
                    .get_series_episodes(&series_id, Some(page))
                    .await
                    .map_err(|error| Error::extension("load episodes", error))
            },
            Message::Loaded,
        )
    }

    fn load_videos(&self, episode_id: &str) -> (Task<Message>, Handle) {
        let extension = self.extension.extension.clone();
        let series_id = self.series_id.clone();
        let episode_id = episode_id.to_owned();

        let task = Task::perform(
            async move {
                extension
                    .get_series_videos(&series_id, &episode_id)
                    .await
                    .map_err(|error| Error::extension("load videos", error))
            },
            Message::VideosLoaded,
        );
        let (task, handle) = task.abortable();

        (task, handle.abort_on_drop())
    }

    fn play(
        &self,
        media: MediaResource,
        episode_number: u32,
        player_path: PathBuf,
    ) -> Task<Message> {
        let playback = self.playback.clone();
        let extension = self.extension.extension.clone();
        let series_id = self.series_id.clone();

        Task::perform(
            async move {
                playback
                    .play(player_path, media, extension, series_id, episode_number)
                    .await
            },
            Message::PlayerStarted,
        )
    }
}

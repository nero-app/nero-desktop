mod episodes;

use iced::widget::{button, column, container, row, scrollable, space, stack, text};
use iced::{padding, Center, Element, Fill, Length, Task};
use nero_extensions::types::Series as SeriesData;
use nero_extensions::Extension;
use rust_i18n::t;
use std::path::Path;

use crate::components::dialog::overlay;
use crate::components::image::cover;
use crate::components::layout::media_layout;
use crate::components::styles;
use crate::components::tabs::tabs;
use crate::components::typography::{self, TextExt};
use crate::error::{Error, Result};
use crate::extensions::LoadedExtension;
use crate::fetch::Fetch;
use crate::icons;
use crate::images::Images;
use crate::player::Playback;
use crate::screens::{Action, Route};
use crate::theme::PALETTE;
use crate::widgets::extension;
use crate::widgets::toolbar::toolbar;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Episodes,
    Related,
}

impl Tab {
    const ALL: [Tab; 2] = [Tab::Episodes, Tab::Related];

    fn label(self) -> text::Fragment<'static> {
        match self {
            Tab::Episodes => t!("media.episodes"),
            Tab::Related => t!("media.more_like_this"),
        }
    }
}

pub struct Series {
    extension: LoadedExtension,
    images: Images,
    tab: Tab,
    series: Fetch<SeriesData>,
    episodes: episodes::Episodes,
    showing_extension: bool,
}

#[derive(Clone)]
pub enum Message {
    SeriesLoaded(Result<SeriesData>),
    ImageLoaded,
    Episodes(episodes::Message),
    PlayFirst,
    ExtensionClicked,
    CloseDialog,
    TabSelected(Tab),
    Navigate(Route),
}

impl Series {
    pub fn new(
        playback: Playback,
        images: Images,
        extension: LoadedExtension,
        series_id: String,
    ) -> (Self, Task<Message>) {
        let (episodes, load_episodes) = episodes::Episodes::new(
            playback,
            images.clone(),
            extension.clone(),
            series_id.clone(),
        );

        let load_series = {
            let extension = extension.extension.clone();

            Task::perform(
                async move {
                    extension
                        .get_series_info(&series_id)
                        .await
                        .map_err(|error| Error::extension("load series information", error))
                },
                Message::SeriesLoaded,
            )
        };

        let screen = Self {
            extension,
            images,
            tab: Tab::Episodes,
            series: Fetch::Loading,
            episodes,
            showing_extension: false,
        };

        (
            screen,
            Task::batch([load_series, load_episodes.map(Message::Episodes)]),
        )
    }

    pub fn update(&mut self, message: Message, player_path: Option<&Path>) -> Action<Message> {
        match message {
            Message::SeriesLoaded(result) => {
                let load_image = result
                    .as_ref()
                    .ok()
                    .and_then(|series| series.poster_resource.clone())
                    .map(|resource| self.images.load(resource, Message::ImageLoaded))
                    .unwrap_or_else(Task::none);
                self.series = result.into();
                Action::run(load_image)
            }
            Message::ImageLoaded => Action::None,
            Message::Episodes(message) => self
                .episodes
                .update(message, player_path)
                .map(Message::Episodes),
            Message::PlayFirst => self.episodes.play_first().map(Message::Episodes),
            Message::ExtensionClicked => {
                self.showing_extension = true;
                Action::None
            }
            Message::CloseDialog => {
                self.showing_extension = false;
                Action::None
            }
            Message::TabSelected(tab) => {
                self.tab = tab;
                Action::None
            }
            Message::Navigate(route) => Action::Navigate(route),
        }
    }

    pub fn view(&self, has_player: bool) -> Element<'_, Message> {
        let poster = self
            .series
            .loaded()
            .and_then(|series| series.poster_resource.as_ref())
            .and_then(|resource| self.images.handle(resource));
        let media = cover(poster, 0.0, 72.0);
        let screen = media_layout(media, self.content());

        stack![screen, self.overlay(has_player)].into()
    }

    fn overlay(&self, has_player: bool) -> Option<Element<'_, Message>> {
        if self.showing_extension {
            return Some(overlay(
                extension::info_dialog(&self.extension, Message::CloseDialog),
                Message::CloseDialog,
            ));
        }

        self.episodes
            .overlay(has_player)
            .map(|overlay| overlay.map(Message::Episodes))
    }

    fn content(&self) -> Element<'_, Message> {
        let body = match &self.series {
            Fetch::Loading => Element::from(text(t!("common.loading")).body()),
            Fetch::Failed(error) => Element::from(text(error.to_string()).body()),
            Fetch::Loaded(series) => Element::from(
                column![
                    self.header(series),
                    tabs(Tab::ALL, self.tab, Tab::label, Message::TabSelected),
                    match self.tab {
                        Tab::Episodes => self.episodes.view().map(Message::Episodes),
                        Tab::Related => space().into(),
                    },
                ]
                .spacing(16)
                .padding(padding::right(32).bottom(32).left(32)),
            ),
        };

        column![
            toolbar(None, |link| Message::Navigate(link.into())),
            scrollable(body).width(Fill).height(Fill)
        ]
        .height(Fill)
        .into()
    }

    fn header<'a>(&'a self, series: &'a SeriesData) -> Element<'a, Message> {
        let title = text(&series.title).title().truncate();
        let extension = self.extension.id.as_ref();

        let subtitle = container(
            row![
                text(series.r#type.as_deref().unwrap_or_default()).label(),
                text("·").label(),
                button(text(extension).label().wrapping(text::Wrapping::None))
                    .on_press(Message::ExtensionClicked)
                    .padding(0)
                    .style(styles::link_button),
            ]
            .spacing(6)
            .align_y(Center),
        )
        .width(Fill)
        .clip(true);

        let play = button(
            container(
                row![
                    icons::play().size(24).color(PALETTE.on_accent),
                    text(t!("media.start_watching")).body(),
                ]
                .spacing(8)
                .align_y(Center),
            )
            .center_x(Fill),
        )
        .on_press(Message::PlayFirst)
        .padding(10)
        .width(Fill)
        .style(styles::primary_button);

        let save = button(icons::bookmark().size(20).color(PALETTE.text_control))
            .padding(10)
            .style(styles::outline_button);

        let share = button(icons::share().size(20).color(PALETTE.text_control))
            .padding(10)
            .style(styles::outline_button);

        let thumbs_up = button(icons::thumbs_up().size(20).color(PALETTE.text_control))
            .padding(10)
            .style(styles::outline_button);

        let synopsis = text(series.synopsis.as_deref().unwrap_or_default())
            .body()
            .line_height(typography::LINE_HEIGHT)
            .width(Fill)
            .height(Length::Fixed(
                typography::BODY * typography::LINE_HEIGHT * 4.0,
            ))
            .wrapping(text::Wrapping::WordOrGlyph)
            .ellipsis(text::Ellipsis::End);

        column![
            title,
            subtitle,
            synopsis,
            row![play, save, share, thumbs_up]
                .spacing(16)
                .align_y(Center),
        ]
        .spacing(16)
        .into()
    }
}

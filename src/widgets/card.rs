use std::borrow::Cow;

use iced::widget::grid::Sizing;
use iced::widget::image;
use iced::widget::{button, column, container, hover, row, text};
use iced::{border, Center, Color, Element, Fill};
use nero_extensions::types::{Episode, Series, Video};
use rust_i18n::t;

use crate::components::image::cover;
use crate::components::styles;
use crate::components::typography::TextExt;
use crate::theme::PALETTE;

const THUMBNAIL_RADIUS: f32 = 8.0;

const CAPTION_GAP: f32 = 4.0;

const SERIES_CARD_PADDING: f32 = 4.0;

pub const EPISODE_GRID_SPACING: f32 = 4.0;

pub const SERIES_GRID_SPACING: f32 = 0.0;

pub const EPISODE_CARD_RATIO: Sizing = Sizing::AspectRatio(3.0 / 2.0);

pub const SERIES_CARD_RATIO: Sizing = Sizing::AspectRatio(2.0 / 3.2);

pub fn episode_card<'a, Message>(
    episode: &'a Episode,
    thumbnail: Option<image::Handle>,
    on_press: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'static,
{
    let number = text(t!("media.episode_short", number = episode.number))
        .label()
        .color(Color::WHITE);

    let scrim = container(number).center(Fill).style(|_| container::Style {
        background: Some(PALETTE.media_scrim.into()),
        border: border::rounded(THUMBNAIL_RADIUS),
        ..container::Style::default()
    });

    let label = match &episode.title {
        Some(title) => Cow::Borrowed(title.as_str()),
        None => t!("media.episode", number = episode.number),
    };

    button(
        column![
            hover(cover(thumbnail, THUMBNAIL_RADIUS, 28.0), scrim),
            text(label).caption()
        ]
        .spacing(CAPTION_GAP),
    )
    .on_press(on_press)
    .padding(0)
    .width(Fill)
    .height(Fill)
    .style(styles::bare_button)
    .into()
}

pub fn series_card<'a, Message>(
    series: &'a Series,
    poster: Option<image::Handle>,
    on_press: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'static,
{
    let poster = column![
        cover(poster, THUMBNAIL_RADIUS, 28.0),
        text(&series.title).caption()
    ];

    button(poster.spacing(CAPTION_GAP))
        .on_press(on_press)
        .padding(SERIES_CARD_PADDING)
        .width(Fill)
        .height(Fill)
        .style(styles::media_card_button)
        .into()
}

pub fn video_card<Message>(video: &Video, on_press: Message) -> Element<'_, Message>
where
    Message: Clone + 'static,
{
    let (width, height) = video.resolution;

    button(
        row![
            text(&video.server).body().width(Fill),
            text(format!("{width}x{height}")).hint(),
        ]
        .align_y(Center),
    )
    .on_press(on_press)
    .width(Fill)
    .padding([8, 12])
    .style(styles::outline_button)
    .into()
}

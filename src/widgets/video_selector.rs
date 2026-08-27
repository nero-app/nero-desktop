use iced::padding;
use iced::widget::{column, container, row, scrollable, text, Container};
use iced::Fill;
use nero_extensions::types::{Episode, MediaResource, Video};
use rust_i18n::t;

use crate::components::dialog::dialog as modal;
use crate::components::image::cover;
use crate::components::typography::TextExt;
use crate::fetch::Fetch;
use crate::images::Images;
use crate::widgets::card::video_card;

pub fn dialog<'a, Message>(
    episode: &'a Episode,
    videos: &'a Fetch<Vec<Video>>,
    has_player: bool,
    images: &'a Images,
    on_select: impl Fn(MediaResource) -> Message + Clone + 'a,
    on_close: Message,
) -> Container<'a, Message>
where
    Message: Clone + 'static,
{
    let number = t!("media.episode_short", number = episode.number);
    let title = match &episode.title {
        Some(title) => format!("{number} · {title}"),
        None => number.into_owned(),
    };

    let preview = column![
        container(cover(
            episode
                .thumbnail_resource
                .as_ref()
                .and_then(|resource| images.handle(resource)),
            4.0,
            48.0,
        ))
        .height(150.0),
        text(episode.description.as_deref().unwrap_or_default()).body(),
    ]
    .spacing(8)
    .padding([0, 16]);

    let sources = if has_player {
        match videos {
            Fetch::Loading => iced::Element::from(text(t!("common.loading")).body()),
            Fetch::Failed(error) => iced::Element::from(text(error.to_string()).body()),
            Fetch::Loaded(videos) => iced::Element::from(
                scrollable(
                    column(videos.iter().map(move |video| {
                        video_card(video, on_select.clone()(video.media_resource.clone()))
                    }))
                    .spacing(6)
                    .padding([0, 16]),
                )
                .width(Fill)
                .height(Fill),
            ),
        }
    } else {
        iced::Element::from(container(text(t!("media.player_not_configured")).body()).center(Fill))
    };

    modal(
        title,
        row![scrollable(preview).width(Fill).height(Fill), sources].padding(padding::vertical(16)),
        on_close,
    )
}

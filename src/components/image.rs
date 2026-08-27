use iced::widget::{container, image, image::Handle};
use iced::{border, ContentFit, Element, Fill};

use crate::icons;
use crate::theme::PALETTE;

pub fn cover<'a, Message: 'a>(
    handle: Option<Handle>,
    radius: f32,
    placeholder_size: f32,
) -> Element<'a, Message> {
    match handle {
        Some(handle) => image(handle)
            .width(Fill)
            .height(Fill)
            .content_fit(ContentFit::Cover)
            .border_radius(radius)
            .into(),
        None => container(
            icons::image_off()
                .size(placeholder_size)
                .color(PALETTE.border),
        )
        .center(Fill)
        .style(move |_| container::Style {
            background: Some(PALETTE.placeholder.into()),
            border: border::rounded(radius),
            ..container::Style::default()
        })
        .into(),
    }
}

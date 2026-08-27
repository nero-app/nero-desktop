use iced::widget::{button, column, container, mouse_area, opaque, row, text, Container};
use iced::{border, Center, Element, Fill, Length};

use crate::components::divider;
use crate::components::styles;
use crate::components::typography::TextExt;
use crate::icons;
use crate::theme::PALETTE;

const MARGIN: u16 = 32;
const MAX_WIDTH: f32 = 880.0;
const MAX_HEIGHT: f32 = 520.0;

pub fn overlay<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    on_blur: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    opaque(
        mouse_area(
            container(opaque(content.into()))
                .center(Fill)
                .padding(MARGIN)
                .style(|_| container::Style {
                    background: Some(PALETTE.scrim.into()),
                    ..container::Style::default()
                }),
        )
        .on_press(on_blur),
    )
}

pub fn dialog<'a, Message>(
    title: impl text::IntoFragment<'a>,
    content: impl Into<Element<'a, Message>>,
    on_close: Message,
) -> Container<'a, Message>
where
    Message: Clone + 'static,
{
    let header = container(
        row![
            text(title).heading().width(Fill),
            button(icons::x().size(18).color(PALETTE.text_control))
                .on_press(on_close)
                .padding(6)
                .style(styles::outline_button),
        ]
        .align_y(Center)
        .spacing(16),
    )
    .padding([8, 16]);

    container(
        column![header, divider(), content.into()]
            .width(Fill)
            .height(Fill),
    )
    .width(Length::Fill.max(MAX_WIDTH))
    .height(Length::Fill.max(MAX_HEIGHT))
    .clip(true)
    .style(|_| container::Style {
        background: Some(PALETTE.surface.into()),
        border: border::rounded(8.0),
        ..container::Style::default()
    })
}

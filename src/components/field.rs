use iced::widget::{container, row, text, text_input};
use iced::{Center, Element, Fill};

use crate::components::styles;
use crate::components::typography;
use crate::icons;
use crate::theme::PALETTE;

pub fn search_field<'a, Message>(
    placeholder: impl text::IntoFragment<'a>,
    value: impl text::IntoFragment<'a>,
    on_input: impl Fn(String) -> Message + 'a,
    on_submit: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let input = text_input(placeholder, value)
        .on_input(on_input)
        .on_submit(on_submit)
        .size(typography::LABEL)
        .padding(0)
        .style(styles::embedded_field);

    container(
        row![icons::search().size(18).color(PALETTE.text_muted), input]
            .align_y(Center)
            .spacing(8),
    )
    .width(Fill)
    .padding([8, 12])
    .style(styles::search_box)
    .into()
}

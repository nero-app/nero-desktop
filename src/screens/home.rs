use iced::widget::{button, column, container, row, space, svg, text};
use iced::{border, Center, Element, Fill};
use rust_i18n::t;

use crate::components::layout::media_layout;
use crate::components::styles;
use crate::components::typography::TextExt;
use crate::icons;
use crate::theme::PALETTE;

const SHOCKED_CAT: &[u8] = include_bytes!("../../assets/shocked_cat.svg");
const CAT_SIZE: f32 = 326.0;

pub fn view<'a, Message: Clone + 'a>(on_search: Message) -> Element<'a, Message> {
    let search = button(
        container(
            row![
                icons::search().size(20).color(PALETTE.on_accent),
                text(t!("media.search_placeholder")).body(),
            ]
            .spacing(8)
            .align_y(Center),
        )
        .center_x(Fill),
    )
    .on_press(on_search)
    .padding(10)
    .width(Fill)
    .style(styles::primary_button);

    let feed = column![
        svg(svg::Handle::from_memory(SHOCKED_CAT))
            .width(CAT_SIZE)
            .height(CAT_SIZE),
        text(t!("common.error_title")).body().center().width(Fill),
        text(t!("common.empty")).body().center().width(Fill),
        search,
    ]
    .spacing(8)
    .align_x(Center)
    .padding([0, 32]);

    let content = container(feed).center(Fill);

    let media = container(space())
        .width(Fill)
        .height(Fill)
        .style(|_| container::Style {
            background: Some(PALETTE.placeholder.into()),
            border: border::rounded(16.0),
            ..container::Style::default()
        });

    media_layout(media, content).into()
}

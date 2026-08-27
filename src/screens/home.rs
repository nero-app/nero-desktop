use iced::widget::{button, column, container, row, space, svg, text};
use iced::{Center, Element, Fill};
use rust_i18n::t;

use crate::components::layout::media_layout;
use crate::components::styles;
use crate::components::typography::TextExt;
use crate::icons;
use crate::screens::{Action, Route};
use crate::theme::PALETTE;
use crate::widgets::toolbar::{toolbar, Link};

const SHOCKED_CAT: &[u8] = include_bytes!("../../assets/shocked_cat.svg");
const CAT_SIZE: f32 = 326.0;

#[derive(Clone)]
pub enum Message {
    Navigate(Route),
}

pub fn update(message: Message) -> Action<Message> {
    match message {
        Message::Navigate(route) => Action::Navigate(route),
    }
}

pub fn view<'a>() -> Element<'a, Message> {
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
    .on_press(Message::Navigate(Route::Search))
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

    let content = column![
        toolbar(Some(Link::Home), |link| Message::Navigate(link.into())),
        container(feed).center(Fill),
    ]
    .height(Fill);

    let media = container(space())
        .width(Fill)
        .height(Fill)
        .style(|_| container::Style {
            background: Some(PALETTE.placeholder.into()),
            ..container::Style::default()
        });

    media_layout(media, content).into()
}

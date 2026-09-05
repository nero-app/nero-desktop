use iced::widget::{button, container, row, text};
use iced::{padding, Center, Element, Fill, Length};
use rust_i18n::t;

use crate::components::field::search_field;
use crate::components::styles;
use crate::components::typography::TextExt;
use crate::icons;
use crate::theme::PALETTE;

const TITLE: &str = concat!(env!("CARGO_PKG_NAME"), "@", env!("CARGO_PKG_VERSION"));

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Link {
    Home,
    Extensions,
    Settings,
}

#[derive(Clone)]
pub enum Message {
    Navigate(Link),
    QueryChanged(String),
    Search,
}

pub fn toolbar<'a>(active: Option<Link>, query: &'a str) -> Element<'a, Message> {
    let navigation_link = |link: Link, label: String| {
        button(text(label).label())
            .on_press(Message::Navigate(link))
            .padding(4)
            .style(if active == Some(link) {
                styles::active_link_button
            } else {
                styles::link_button
            })
    };

    let navigation = row![
        navigation_link(Link::Home, t!("nav.home").to_string()),
        navigation_link(Link::Extensions, t!("nav.extensions").to_string()),
        navigation_link(Link::Settings, t!("nav.settings").to_string()),
    ]
    .spacing(12)
    .align_y(Center);
    let brand = text(TITLE).label();
    let search = container(search_field(
        t!("media.search_placeholder"),
        query,
        Message::QueryChanged,
        Message::Search,
    ))
    .width(Length::Fill.max(240.0));
    let bookmark = button(icons::bookmark().size(22).color(PALETTE.text_link))
        .padding(4)
        .style(styles::link_button);
    let actions = row![search, bookmark]
        .spacing(12)
        .align_y(Center)
        .width(Length::Fill.max(282.0));

    container(
        row![
            container(brand).width(Fill),
            container(navigation).center_x(Fill),
            container(actions).align_right(Fill),
        ]
        .width(Fill)
        .align_y(Center)
        .padding(padding::horizontal(24)),
    )
    .style(|_| container::Style {
        background: Some(PALETTE.surface.into()),
        ..container::Style::default()
    })
    .into()
}

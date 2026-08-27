use iced::widget::{button, row, space, text, Button, Row};
use iced::{padding, Center, Fill};
use rust_i18n::t;

use crate::components::styles;
use crate::components::typography::TextExt;
use crate::icons;
use crate::theme::PALETTE;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Link {
    Home,
    Extensions,
    Settings,
    Search,
}

pub fn toolbar<'a, Message>(
    active: Option<Link>,
    on_navigate: impl Fn(Link) -> Message,
) -> Row<'a, Message>
where
    Message: Clone + 'static,
{
    let navigation_link = |link: Link, label: String| -> Button<'a, Message> {
        button(text(label).label())
            .on_press(on_navigate(link))
            .padding(4)
            .style(if active == Some(link) {
                styles::active_link_button
            } else {
                styles::link_button
            })
    };

    row![
        navigation_link(Link::Home, t!("nav.home").to_string()),
        navigation_link(Link::Extensions, t!("nav.extensions").to_string()),
        navigation_link(Link::Settings, t!("nav.settings").to_string()),
        space().width(Fill),
        button(icons::search().size(22).color(PALETTE.text_link))
            .on_press(on_navigate(Link::Search))
            .padding(4)
            .style(if active == Some(Link::Search) {
                styles::active_link_button
            } else {
                styles::link_button
            }),
    ]
    .spacing(16)
    .align_y(Center)
    .padding(padding::top(16).right(32).bottom(8).left(32))
}

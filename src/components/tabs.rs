use iced::widget::{button, column, container, row, rule, space, text, Button, Row};
use iced::{Element, Fill, Theme};

use crate::components::typography::TextExt;
use crate::icons;
use crate::theme::PALETTE;

const INDICATOR_SIZE: f32 = 12.0;
const ROW_HEIGHT: f32 = 26.0;

pub fn tabs<'a, Tab, Label, Message>(
    options: impl IntoIterator<Item = Tab>,
    selected: Tab,
    label: impl Fn(Tab) -> Label,
    on_select: impl Fn(Tab) -> Message,
) -> Row<'a, Message>
where
    Tab: Copy + Eq,
    Label: text::IntoFragment<'a>,
    Message: Clone + 'static,
{
    row(options
        .into_iter()
        .map(|tab| trigger(label(tab), tab == selected, on_select(tab)).into()))
    .width(Fill)
}

fn trigger<'a, Message>(
    label: impl text::IntoFragment<'a>,
    active: bool,
    on_press: Message,
) -> Button<'a, Message>
where
    Message: Clone + 'static,
{
    let line_color = if active {
        PALETTE.text
    } else {
        PALETTE.separator
    };

    let line = rule::horizontal(2).style(move |theme| rule::Style {
        color: line_color,
        ..rule::default(theme)
    });

    let indicator = if active {
        Element::from(
            container(
                icons::triangle_down()
                    .size(INDICATOR_SIZE)
                    .color(PALETTE.text),
            )
            .center_x(Fill),
        )
    } else {
        Element::from(space().height(INDICATOR_SIZE))
    };

    let title = if active {
        text(label).label_strong()
    } else {
        text(label).label()
    }
    .center()
    .width(Fill)
    .height(ROW_HEIGHT);

    button(column![line, indicator, title].width(Fill))
        .on_press(on_press)
        .padding(0)
        .width(Fill)
        .style(if active { tab_active } else { tab_idle })
}

fn tab_active(_theme: &Theme, _status: button::Status) -> button::Style {
    button::Style {
        background: None,
        text_color: PALETTE.text,
        ..button::Style::default()
    }
}

fn tab_idle(_theme: &Theme, status: button::Status) -> button::Style {
    button::Style {
        background: None,
        text_color: match status {
            button::Status::Hovered | button::Status::Pressed => PALETTE.text_link,
            _ => PALETTE.text_muted,
        },
        ..button::Style::default()
    }
}

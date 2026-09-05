use iced::widget::{column, container, row, Column, Row};
use iced::{padding, Element, Fill, FillPortion};

const TOOLBAR_HEIGHT: f32 = 64.0;

pub fn main_layout<'a, Message: 'a>(
    toolbar: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
) -> Column<'a, Message> {
    column![
        container(toolbar.into())
            .width(Fill)
            .center_y(TOOLBAR_HEIGHT),
        container(content.into()).width(Fill).height(Fill),
    ]
    .width(Fill)
    .height(Fill)
}

pub fn media_layout<'a, Message: 'a>(
    media: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
) -> Row<'a, Message> {
    row![
        container(media.into())
            .width(FillPortion(2))
            .height(Fill)
            .padding(padding::left(24).bottom(24))
            .clip(true),
        container(content.into()).width(FillPortion(3)).height(Fill),
    ]
    .height(Fill)
}

pub fn sidebar_layout<'a, Message: 'a>(
    main: impl Into<Element<'a, Message>>,
    sidebar: impl Into<Element<'a, Message>>,
) -> Row<'a, Message> {
    row![
        container(main.into()).width(FillPortion(4)).height(Fill),
        container(sidebar.into()).width(FillPortion(2)).height(Fill),
    ]
    .height(Fill)
}

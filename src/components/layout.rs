use iced::widget::{container, row, Row};
use iced::{Element, Fill, FillPortion};

pub fn media_layout<'a, Message: 'a>(
    media: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
) -> Row<'a, Message> {
    row![
        container(media.into())
            .width(FillPortion(2))
            .height(Fill)
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

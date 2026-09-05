use iced::widget::{button, column, row, text};
use iced::{Center, Element, Fill};

use crate::components::styles;
use crate::components::typography::TextExt;
use crate::icons;
use crate::interactions::notification::{Message, Notification};
use crate::theme::PALETTE;

pub fn view(notification: &Notification) -> Element<'_, Message> {
    row![
        column![
            text(&notification.message).label_strong(),
            notification
                .detail
                .as_ref()
                .map(|detail| text(detail).label()),
            text(notification.extension_id.as_ref()).hint(),
        ]
        .width(Fill)
        .spacing(4),
        button(icons::x().size(16).color(PALETTE.text_muted))
            .on_press(Message::Dismiss(notification.id))
            .padding(4)
            .style(styles::bare_button),
    ]
    .align_y(Center)
    .spacing(12)
    .into()
}

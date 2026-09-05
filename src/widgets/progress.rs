use iced::animation::Easing;
use iced::widget::{button, column, progress_bar, row, text, transition};
use iced::{Animation, Center, Element, Fill};
use rust_i18n::t;

use crate::components::styles;
use crate::components::typography::TextExt;
use crate::interactions::progress::{Message, ProgressEntry, ProgressId};

pub fn view<'a>(id: &'a ProgressId, progress: &'a ProgressEntry) -> Element<'a, Message> {
    let status = if progress.cancelling {
        text(t!("interactions.cancelling")).label()
    } else if let Some(message) = &progress.message {
        text(message).label()
    } else {
        text(t!("common.loading")).label()
    };
    let indicator = progress.percent.map(|percent| {
        column![
            transition(
                f32::from(percent),
                || Animation::new(0.0).quick().easing(Easing::EaseOut),
                |animation, now| progress_bar(0.0..=100.0, animation.interpolate(now)),
            )
            .key(("progress-bar", id.clone())),
            text(format!("{percent}%")).hint(),
        ]
        .spacing(4)
    });
    let cancel = (progress.cancellable && !progress.cancelling).then(|| {
        button(text(t!("common.cancel")).label())
            .on_press(Message::Cancel(id.clone()))
            .padding([6, 10])
            .style(styles::outline_button)
    });

    column![
        row![
            column![
                text(&progress.title).label_strong(),
                text(id.extension_id().as_ref()).hint(),
            ]
            .width(Fill)
            .spacing(2),
            cancel,
        ]
        .align_y(Center)
        .spacing(8),
        status,
        indicator,
    ]
    .spacing(8)
    .into()
}

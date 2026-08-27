use iced::widget::{rule, Rule};

use crate::theme::PALETTE;

pub fn divider<'a>() -> Rule<'a> {
    rule::horizontal(1).style(|theme| rule::Style {
        color: PALETTE.separator,
        ..rule::default(theme)
    })
}

pub fn vertical_divider<'a>() -> Rule<'a> {
    rule::vertical(1).style(|theme| rule::Style {
        color: PALETTE.separator,
        ..rule::default(theme)
    })
}

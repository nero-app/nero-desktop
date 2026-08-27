use iced::font::Weight;
use iced::widget::{text, Text};
use iced::{Fill, Font};

use crate::theme::{self, PALETTE};

pub const LINE_HEIGHT: f32 = 1.3;

pub const TITLE: f32 = 36.0;
pub const SECTION: f32 = 20.0;
pub const HEADING: f32 = 18.0;
pub const BODY: f32 = 16.0;
pub const LABEL: f32 = 14.0;
pub const HINT: f32 = 12.0;

fn weighted(weight: Weight) -> Font {
    Font {
        weight,
        ..theme::UI_FONT
    }
}

pub trait TextExt {
    fn title(self) -> Self;
    fn section(self) -> Self;
    fn heading(self) -> Self;
    fn body(self) -> Self;
    fn label(self) -> Self;
    fn label_strong(self) -> Self;
    fn hint(self) -> Self;
    fn truncate(self) -> Self;
    fn caption(self) -> Self;
}

impl TextExt for Text<'_> {
    fn title(self) -> Self {
        self.size(TITLE).font(weighted(Weight::Bold))
    }

    fn section(self) -> Self {
        self.size(SECTION).font(weighted(Weight::Semibold))
    }

    fn heading(self) -> Self {
        self.size(HEADING).font(weighted(Weight::Medium))
    }

    fn body(self) -> Self {
        self.size(BODY)
    }

    fn label(self) -> Self {
        self.size(LABEL)
    }

    fn label_strong(self) -> Self {
        self.label().font(weighted(Weight::Semibold))
    }

    fn hint(self) -> Self {
        self.size(HINT).color(PALETTE.text_muted)
    }

    fn truncate(self) -> Self {
        self.width(Fill)
            .wrapping(text::Wrapping::None)
            .ellipsis(text::Ellipsis::End)
    }

    fn caption(self) -> Self {
        self.label().truncate().center().height(LABEL * LINE_HEIGHT)
    }
}

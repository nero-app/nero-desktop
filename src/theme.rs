use iced::theme::palette::Seed;
use iced::{Color, Font, Theme};

pub const UI_FONT: Font = Font::new("Inter");

pub const UI_FONTS: [&[u8]; 4] = [
    include_bytes!("../assets/fonts/Inter-Regular.ttf"),
    include_bytes!("../assets/fonts/Inter-Medium.ttf"),
    include_bytes!("../assets/fonts/Inter-SemiBold.ttf"),
    include_bytes!("../assets/fonts/Inter-Bold.ttf"),
];

const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a: 1.0,
    }
}

const fn rgba(r: u8, g: u8, b: u8, a: f32) -> Color {
    Color { a, ..rgb(r, g, b) }
}

pub struct Palette {
    pub surface: Color,
    pub raised: Color,
    pub raised_hovered: Color,
    pub placeholder: Color,
    pub separator: Color,
    pub border: Color,
    pub border_hovered: Color,
    pub text: Color,
    pub text_muted: Color,
    pub text_control: Color,
    pub text_link: Color,
    pub accent: Color,
    pub accent_hovered: Color,
    pub on_accent: Color,
    pub scrim: Color,
    pub media_scrim: Color,
}

pub const PALETTE: Palette = Palette {
    surface: rgb(255, 255, 255),
    raised: rgb(245, 245, 245),
    raised_hovered: rgb(212, 212, 212),
    placeholder: rgb(229, 229, 229),
    separator: rgb(229, 229, 229),
    border: rgb(212, 212, 212),
    border_hovered: rgb(163, 163, 163),
    text: rgb(23, 23, 23),
    text_muted: rgb(115, 115, 115),
    text_control: rgb(64, 64, 64),
    text_link: rgb(38, 38, 38),
    accent: rgb(254, 215, 170),
    accent_hovered: rgb(253, 186, 116),
    on_accent: rgb(0, 0, 0),
    scrim: rgba(0, 0, 0, 0.5),
    media_scrim: rgba(0, 0, 0, 0.2),
};

pub fn nero() -> Theme {
    Theme::custom(
        "Nero",
        Seed {
            background: PALETTE.surface,
            text: PALETTE.text,
            primary: PALETTE.accent,
            success: rgb(5, 150, 105),
            warning: rgb(217, 119, 6),
            danger: rgb(220, 38, 38),
        },
    )
}

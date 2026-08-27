// Generated automatically by iced_lucide at build time.
// Do not edit manually.
// 57696cd1daab19d998e972c7864671f4747d584dbd80aced54b27cb88ebb0ff2
use iced::widget::{text, Text};
use iced::Font;

pub const FONT: &[u8] = include_bytes!("../fonts/lucide.ttf");

/// All icons as `(name, codepoint_str)` pairs.
/// Use this to populate an icon-picker widget.
#[allow(dead_code)]
pub const ALL_ICONS: &[(&str, &str)] = &[
    ("blocks", "\u{E4FA}"),
    ("bookmark", "\u{E060}"),
    ("image_off", "\u{E1C0}"),
    ("play", "\u{E13C}"),
    ("search", "\u{E151}"),
    ("share", "\u{E156}"),
    ("thumbs_up", "\u{E18A}"),
    ("triangle_down", "\u{E06D}"),
    ("x", "\u{E1B2}"),
];

pub fn blocks<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    icon("\u{E4FA}")
}

pub fn bookmark<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    icon("\u{E060}")
}

pub fn image_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    icon("\u{E1C0}")
}

pub fn play<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    icon("\u{E13C}")
}

pub fn search<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    icon("\u{E151}")
}

pub fn share<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    icon("\u{E156}")
}

pub fn thumbs_up<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    icon("\u{E18A}")
}

pub fn triangle_down<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    icon("\u{E06D}")
}

pub fn x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    icon("\u{E1B2}")
}

/// Render any Lucide icon by its codepoint string.
/// Use this together with [`ALL_ICONS`] to display icons dynamically:
/// ```ignore
/// for (name, cp) in ALL_ICONS {
///     button(render(cp)).on_press(Msg::Pick(name.to_string()))
///
/// ```
pub fn render<'a, Theme, Renderer>(codepoint: &'a str) -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    text(codepoint).font(Font::new("lucide")).line_height(1.0)
}

fn icon<'a, Theme, Renderer>(codepoint: &'a str) -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    render(codepoint)
}

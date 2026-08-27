use iced::widget::{button, container, pick_list, text_input, toggler};
use iced::{border, Background, Border, Color, Theme};

use crate::theme::PALETTE;

pub fn primary_button(_theme: &Theme, status: button::Status) -> button::Style {
    let background = match status {
        button::Status::Hovered | button::Status::Pressed => PALETTE.accent_hovered,
        button::Status::Disabled => PALETTE.accent.scale_alpha(0.5),
        button::Status::Active => PALETTE.accent,
    };

    button::Style {
        background: Some(background.into()),
        text_color: PALETTE.on_accent,
        border: border::rounded(6.0),
        ..button::Style::default()
    }
}

pub fn outline_button(_theme: &Theme, status: button::Status) -> button::Style {
    let (border_color, background): (Color, Option<Background>) = match status {
        button::Status::Hovered => (PALETTE.border_hovered, Some(PALETTE.raised.into())),
        button::Status::Disabled => (PALETTE.border.scale_alpha(0.5), None),
        _ => (PALETTE.border, None),
    };

    button::Style {
        background,
        text_color: PALETTE.text_control,
        border: Border {
            color: border_color,
            width: 1.0,
            radius: 8.0.into(),
        },
        ..button::Style::default()
    }
}

pub fn link_button(_theme: &Theme, status: button::Status) -> button::Style {
    button::Style {
        background: None,
        text_color: match status {
            button::Status::Hovered | button::Status::Pressed => PALETTE.text_muted,
            button::Status::Disabled => PALETTE.text_link.scale_alpha(0.5),
            button::Status::Active => PALETTE.text_link,
        },
        border: border::rounded(6.0),
        ..button::Style::default()
    }
}

pub fn active_link_button(_theme: &Theme, _status: button::Status) -> button::Style {
    button::Style {
        background: None,
        text_color: PALETTE.text,
        border: border::rounded(6.0),
        ..button::Style::default()
    }
}

pub fn card_button(_theme: &Theme, status: button::Status) -> button::Style {
    button::Style {
        background: match status {
            button::Status::Hovered => Some(PALETTE.raised.into()),
            _ => None,
        },
        text_color: PALETTE.text,
        border: border::rounded(8.0),
        ..button::Style::default()
    }
}

pub fn media_card_button(_theme: &Theme, status: button::Status) -> button::Style {
    button::Style {
        background: match status {
            button::Status::Hovered | button::Status::Pressed => {
                Some(PALETTE.raised_hovered.into())
            }
            _ => None,
        },
        text_color: PALETTE.text,
        border: border::rounded(6.0),
        ..button::Style::default()
    }
}

pub fn bare_button(_theme: &Theme, _status: button::Status) -> button::Style {
    button::Style {
        background: None,
        text_color: PALETTE.text,
        border: border::rounded(4.0),
        ..button::Style::default()
    }
}

pub fn search_box(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(PALETTE.raised.into()),
        border: border::rounded(8.0),
        ..container::Style::default()
    }
}

pub fn text_field(_theme: &Theme, _status: text_input::Status) -> text_input::Style {
    text_input::Style {
        background: PALETTE.surface.into(),
        border: Border {
            color: PALETTE.border,
            width: 1.0,
            radius: 6.0.into(),
        },
        placeholder: PALETTE.text_muted,
        value: PALETTE.text,
        selection: PALETTE.accent_hovered,
    }
}

pub fn embedded_field(theme: &Theme, status: text_input::Status) -> text_input::Style {
    text_input::Style {
        background: Color::TRANSPARENT.into(),
        border: Border::default(),
        ..text_field(theme, status)
    }
}

pub fn select(_theme: &Theme, status: pick_list::Status) -> pick_list::Style {
    let border_color = match status {
        pick_list::Status::Hovered | pick_list::Status::Opened { .. } => PALETTE.border_hovered,
        _ => PALETTE.border,
    };

    pick_list::Style {
        text_color: PALETTE.text_control,
        placeholder_color: PALETTE.text_muted,
        handle_color: PALETTE.border_hovered,
        background: PALETTE.surface.into(),
        border: Border {
            color: border_color,
            width: 1.0,
            radius: 6.0.into(),
        },
    }
}

pub fn toggle(_theme: &Theme, status: toggler::Status) -> toggler::Style {
    let background = match status {
        toggler::Status::Active { is_toggled } | toggler::Status::Hovered { is_toggled }
            if is_toggled =>
        {
            PALETTE.accent
        }
        _ => PALETTE.border,
    };

    toggler::Style {
        background: background.into(),
        background_border_width: 0.0,
        background_border_color: PALETTE.border,
        foreground: PALETTE.surface.into(),
        foreground_border_width: 0.0,
        foreground_border_color: PALETTE.surface,
        text_color: None,
        border_radius: None,
        padding_ratio: 0.0,
    }
}

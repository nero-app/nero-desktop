mod app;
mod components;
mod error;
mod extensions;
mod fetch;
mod i18n;
mod icons;
mod images;
mod interactions;
mod media;
mod pagination;
mod picker;
mod player;
mod preferences;
mod screens;
mod server;
mod theme;
mod widgets;

use iced::Size;
use rust_i18n::i18n;

use crate::app::State;

i18n!("locales", fallback = "en");

fn main() -> iced::Result {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "nero_desktop=debug,nero_extensions=info,warn".into()),
        )
        .init();

    let window = iced::window::Settings {
        size: Size::new(1024.0, 640.0),
        min_size: Some(Size::new(768.0, 480.0)),
        max_size: Some(Size::new(1280.0, 800.0)),
        #[cfg(target_os = "macos")]
        platform_specific: iced::window::settings::PlatformSpecific {
            title_hidden: true,
            titlebar_transparent: true,
            fullsize_content_view: true,
        },
        ..iced::window::Settings::default()
    };

    let app = iced::application(State::new, State::update, State::view)
        .subscription(State::subscription)
        .theme(theme::nero())
        .window(window)
        .centered()
        .default_font(theme::UI_FONT)
        .font(icons::FONT);

    theme::UI_FONTS
        .into_iter()
        .fold(app, |app, font| app.font(font))
        .run()
}

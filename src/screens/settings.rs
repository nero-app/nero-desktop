mod app;
mod extensions;

use std::path::Path;
use std::sync::Arc;

use iced::widget::{button, column, scrollable, text};
use iced::{padding, Element, Fill};
use rust_i18n::t;

use crate::components::layout::sidebar_layout;
use crate::components::styles;
use crate::components::typography::TextExt;
use crate::extensions::Registry;
use crate::i18n::Language;
use crate::preferences::{MediaProxyPreferences, PreferenceAction};
use crate::screens::SettingsTab;

enum Section {
    App,
    Extensions(Box<extensions::Extensions>),
}

impl Section {
    fn new(extensions: Arc<Registry>, tab: SettingsTab) -> Self {
        match tab {
            SettingsTab::App => Section::App,
            SettingsTab::Extensions => {
                Section::Extensions(Box::new(extensions::Extensions::new(extensions)))
            }
        }
    }

    fn tab(&self) -> SettingsTab {
        match self {
            Section::App => SettingsTab::App,
            Section::Extensions(_) => SettingsTab::Extensions,
        }
    }

    fn heading(&self) -> (text::Fragment<'static>, text::Fragment<'static>) {
        match self {
            Section::App => (t!("settings.app.title"), t!("settings.app.subtitle")),
            Section::Extensions(_) => (
                t!("settings.extensions.title"),
                t!("settings.extensions.subtitle"),
            ),
        }
    }
}

pub struct Settings {
    extensions: Arc<Registry>,
    section: Section,
}

#[derive(Clone)]
pub enum Message {
    App(app::Message),
    Extensions(extensions::Message),
    TabSelected(SettingsTab),
}

pub type Action<Message> = crate::screens::Action<Message, PreferenceAction>;

impl Settings {
    pub fn new(extensions: Arc<Registry>, tab: SettingsTab) -> Self {
        Self {
            section: Section::new(extensions.clone(), tab),
            extensions,
        }
    }

    pub fn update(&mut self, message: Message) -> Action<Message> {
        let message = match message {
            Message::TabSelected(tab) if tab == self.section.tab() => return Action::None,
            Message::TabSelected(tab) => {
                self.section = Section::new(self.extensions.clone(), tab);
                return Action::None;
            }
            message => message,
        };

        match (&mut self.section, message) {
            (Section::App, Message::App(message)) => app::update(message).map(Message::App),
            (Section::Extensions(section), Message::Extensions(message)) => {
                section.update(message).map(Message::Extensions)
            }
            _ => Action::None,
        }
    }

    pub fn view<'a>(
        &'a self,
        language: Language,
        player_path: Option<&'a Path>,
        media_proxy: &'a MediaProxyPreferences,
    ) -> Element<'a, Message> {
        sidebar_layout(
            self.content(language, player_path, media_proxy),
            self.sidebar(),
        )
        .into()
    }

    pub fn tab(&self) -> SettingsTab {
        self.section.tab()
    }

    pub fn overlay(&self) -> Option<Element<'_, Message>> {
        match &self.section {
            Section::Extensions(section) => section
                .overlay()
                .map(|overlay| overlay.map(Message::Extensions)),
            _ => None,
        }
    }

    fn content<'a>(
        &'a self,
        language: Language,
        player_path: Option<&'a Path>,
        media_proxy: &'a MediaProxyPreferences,
    ) -> Element<'a, Message> {
        let (title, description) = self.section.heading();

        let body = match &self.section {
            Section::App => app::view(language, player_path, media_proxy).map(Message::App),
            Section::Extensions(section) => section.view().map(Message::Extensions),
        };

        let content = column![
            column![text(title).title(), text(description).label()],
            body
        ]
        .spacing(32)
        .padding(padding::right(32).bottom(32).left(32));

        scrollable(content).width(Fill).height(Fill).into()
    }

    fn sidebar(&self) -> Element<'_, Message> {
        let current = self.section.tab();

        let link = |tab: SettingsTab| {
            button(text(tab.label()).body().width(Fill))
                .on_press(Message::TabSelected(tab))
                .width(Fill)
                .padding([6, 8])
                .style(if tab == current {
                    styles::active_link_button
                } else {
                    styles::link_button
                })
        };

        column(SettingsTab::ALL.map(|tab| link(tab).into()))
            .spacing(4)
            .width(Fill)
            .height(Fill)
            .padding(padding::right(32).bottom(32).left(16))
            .into()
    }
}

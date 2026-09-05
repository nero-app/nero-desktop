pub mod home;
pub mod search;
pub mod series;
pub mod settings;

use std::convert::Infallible;

use iced::widget::text;
use iced::Task;
use rust_i18n::t;

use crate::extensions::LoadedExtension;
use crate::widgets::toolbar::Link;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsTab {
    App,
    Extensions,
}

impl SettingsTab {
    pub const ALL: [SettingsTab; 2] = [SettingsTab::App, SettingsTab::Extensions];

    pub fn label(self) -> text::Fragment<'static> {
        match self {
            SettingsTab::App => t!("nav.app"),
            SettingsTab::Extensions => t!("nav.extensions"),
        }
    }
}

#[derive(Clone)]
pub enum Route {
    Home,
    Search,
    Series {
        extension: LoadedExtension,
        series_id: String,
    },
    Settings(SettingsTab),
}

impl From<Link> for Route {
    fn from(link: Link) -> Self {
        match link {
            Link::Home => Route::Home,
            Link::Extensions => Route::Settings(SettingsTab::Extensions),
            Link::Settings => Route::Settings(SettingsTab::App),
        }
    }
}

pub enum Action<Message, Output = Infallible> {
    None,
    Run(Task<Message>),
    Navigate(Route),
    Emit(Output),
}

impl<Message: Send + 'static, Output> Action<Message, Output> {
    pub fn run(task: Task<Message>) -> Self {
        Action::Run(task)
    }

    pub fn emit(output: Output) -> Self {
        Action::Emit(output)
    }

    pub fn map<Mapped: Send + 'static>(
        self,
        wrap: impl Fn(Message) -> Mapped + Send + 'static,
    ) -> Action<Mapped, Output> {
        match self {
            Action::None => Action::None,
            Action::Run(task) => Action::Run(task.map(wrap)),
            Action::Navigate(route) => Action::Navigate(route),
            Action::Emit(output) => Action::Emit(output),
        }
    }
}

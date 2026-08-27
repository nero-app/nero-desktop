use std::net::Ipv4Addr;
use std::sync::Arc;

use iced::widget::{container, text};
use iced::{Element, Fill, Task};
use reqwest::Client;
use rust_i18n::t;
use tokio::net::TcpListener;
use url::Url;

use crate::components::typography::TextExt;
use crate::error::{Error, Result};
use crate::extensions::{default_cache_dir, Registry};
use crate::i18n;
use crate::images::Images;
use crate::media::Media;
use crate::player::Playback;
use crate::preferences::{MediaProxyPreferences, PreferenceAction, Preferences};
use crate::screens::{home, search, series, settings, Action, Route};
use crate::server::Server;

enum Screen {
    Home,
    Search(search::Search),
    Series(Box<series::Series>),
    Settings(settings::Settings),
}

enum Status {
    Starting,
    Failed(Error),
    Ready(Boot),
}

pub struct Boot {
    extensions: Arc<Registry>,
    images: Images,
    media: Media,
    playback: Playback,
}

impl Boot {
    async fn new(preferences: MediaProxyPreferences) -> Result<Self> {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))
            .await
            .map_err(|error| Error::Server(error.to_string()))?;
        let address = listener
            .local_addr()
            .map_err(|error| Error::Server(error.to_string()))?;
        let media_base_url = Url::parse(&format!("http://{address}/"))
            .map_err(|error| Error::Url(error.to_string()))?;

        let http_client = Client::new();
        let media = Media::new(
            http_client.clone(),
            preferences.torrent_enabled,
            preferences.torrent_output_folder,
        )
        .await?;
        let server = Server::new(listener).extend(media.proxy().router());

        let extensions = Arc::new(Registry::default());
        let images = Images::new(http_client.clone(), default_cache_dir().join("images")).await?;

        tokio::spawn(async move {
            if let Err(error) = server.run().await {
                tracing::error!(%error, "the app server stopped serving");
            }
        });

        Ok(Self {
            extensions,
            images,
            playback: Playback::new(http_client, media.clone(), media_base_url),
            media,
        })
    }
}

pub struct State {
    status: Status,
    screen: Screen,
    preferences: Preferences,
}

pub enum Message {
    Booted(Result<Boot>),
    MediaConfigured(Result<PreferenceAction>),
    Home(home::Message),
    Search(search::Message),
    Series(series::Message),
    Settings(settings::Message),
}

impl State {
    pub fn new() -> (Self, Task<Message>) {
        let preferences = Preferences::default();
        let media_preferences = preferences.media_proxy().clone();
        let state = Self {
            status: Status::Starting,
            screen: Screen::Home,
            preferences,
        };

        let task = Task::perform(Boot::new(media_preferences), Message::Booted);

        (state, task)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Booted(Ok(boot)) => {
                i18n::set_language(self.preferences.language());
                self.status = Status::Ready(boot);
                return Task::none();
            }
            Message::Booted(Err(error)) => {
                tracing::error!(%error, "the application runtime failed to start");
                self.status = Status::Failed(error);

                return Task::none();
            }
            Message::MediaConfigured(Ok(action)) => {
                self.preferences.update(action);
                return Task::none();
            }
            Message::MediaConfigured(Err(error)) => {
                tracing::error!(%error, "failed to update media settings");
                return Task::none();
            }
            _ if !matches!(self.status, Status::Ready(_)) => return Task::none(),
            _ => {}
        }

        let message = match (&mut self.screen, message) {
            (Screen::Settings(screen), Message::Settings(message)) => {
                let action = screen.update(message);
                return self.update_settings(action);
            }
            (_, message) => message,
        };

        let action = match (&mut self.screen, message) {
            (Screen::Home, Message::Home(message)) => home::update(message).map(Message::Home),
            (Screen::Search(screen), Message::Search(message)) => {
                screen.update(message).map(Message::Search)
            }
            (Screen::Series(screen), Message::Series(message)) => screen
                .update(message, self.preferences.player_path())
                .map(Message::Series),
            _ => Action::None,
        };

        match action {
            Action::None => Task::none(),
            Action::Run(task) => task,
            Action::Navigate(route) => {
                let (next, task) = self.open(route);
                self.screen = next;

                task
            }
            Action::Emit(never) => match never {},
        }
    }

    fn update_settings(&mut self, action: settings::Action<settings::Message>) -> Task<Message> {
        match action {
            Action::None => Task::none(),
            Action::Run(task) => task.map(Message::Settings),
            Action::Navigate(route) => {
                let (next, task) = self.open(route);
                self.screen = next;
                task
            }
            Action::Emit(PreferenceAction::TorrentEnabled(enabled)) => {
                let Status::Ready(boot) = &self.status else {
                    return Task::none();
                };
                let media = boot.media.clone();
                let output_dir = self.preferences.media_proxy().torrent_output_folder.clone();

                Task::perform(
                    async move {
                        media
                            .configure(enabled, output_dir)
                            .await
                            .map(|_| PreferenceAction::TorrentEnabled(enabled))
                    },
                    Message::MediaConfigured,
                )
            }
            Action::Emit(PreferenceAction::TorrentOutputFolder(output_dir)) => {
                let Status::Ready(boot) = &self.status else {
                    return Task::none();
                };
                let media = boot.media.clone();
                let enabled = self.preferences.media_proxy().torrent_enabled;

                Task::perform(
                    async move {
                        media
                            .configure(enabled, output_dir.clone())
                            .await
                            .map(|_| PreferenceAction::TorrentOutputFolder(output_dir))
                    },
                    Message::MediaConfigured,
                )
            }
            Action::Emit(action) => {
                if let Some(language) = self.preferences.update(action) {
                    i18n::set_language(language);
                }

                Task::none()
            }
        }
    }

    fn open(&self, route: Route) -> (Screen, Task<Message>) {
        let Status::Ready(boot) = &self.status else {
            unreachable!("open() called before boot finished")
        };

        match route {
            Route::Home => (Screen::Home, Task::none()),
            Route::Search => {
                let (screen, task) =
                    search::Search::new(boot.extensions.as_ref(), boot.images.clone());
                (Screen::Search(screen), task.map(Message::Search))
            }
            Route::Series {
                extension,
                series_id,
            } => {
                let (screen, task) = series::Series::new(
                    boot.playback.clone(),
                    boot.images.clone(),
                    extension,
                    series_id,
                );

                (Screen::Series(Box::new(screen)), task.map(Message::Series))
            }
            Route::Settings(tab) => {
                let screen = settings::Settings::new(boot.extensions.clone(), tab);
                (Screen::Settings(screen), Task::none())
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        match &self.status {
            Status::Starting => container(text(t!("common.loading")).body())
                .center(Fill)
                .into(),
            Status::Failed(error) => container(text(error.to_string()).body())
                .center(Fill)
                .into(),
            Status::Ready(_) => match &self.screen {
                Screen::Home => home::view().map(Message::Home),
                Screen::Search(screen) => screen.view().map(Message::Search),
                Screen::Series(screen) => screen
                    .view(self.preferences.player_path().is_some())
                    .map(Message::Series),
                Screen::Settings(screen) => screen
                    .view(
                        self.preferences.language(),
                        self.preferences.player_path(),
                        self.preferences.media_proxy(),
                    )
                    .map(Message::Settings),
            },
        }
    }
}

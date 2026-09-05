use std::net::Ipv4Addr;
use std::sync::Arc;

use iced::widget::{container, stack, text};
use iced::{Element, Fill, Subscription, Task};
use reqwest::Client;
use rust_i18n::t;
use tokio::net::TcpListener;
use url::Url;

use crate::components::layout::main_layout;
use crate::components::typography::TextExt;
use crate::error::{Error, Result};
use crate::extensions::{default_cache_dir, Registry};
use crate::i18n;
use crate::images::Images;
use crate::interactions;
use crate::media::Media;
use crate::player::Playback;
use crate::preferences::{MediaProxyPreferences, PreferenceAction, Preferences};
use crate::screens::{home, search, series, settings, Action, Route, SettingsTab};
use crate::server::Server;
use crate::widgets::toolbar::{self, Link};

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
    async fn new(
        preferences: MediaProxyPreferences,
        interaction_transport: Arc<interactions::Transport>,
    ) -> Result<Self> {
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

        let extensions = Arc::new(Registry::new(interaction_transport));
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
    search_query: String,
    interactions: interactions::InteractionState,
}

pub enum Message {
    Navigate(Route),
    Booted(Result<Boot>),
    MediaConfigured(Result<PreferenceAction>),
    CallbackReceived(String),
    Interaction(interactions::Message),
    Toolbar(toolbar::Message),
    Search(search::Message),
    Series(series::Message),
    Settings(settings::Message),
}

impl State {
    pub fn new() -> (Self, Task<Message>) {
        let preferences = Preferences::default();
        let media_preferences = preferences.media_proxy().clone();
        let (interactions, interaction_transport) = interactions::InteractionState::new();
        let state = Self {
            status: Status::Starting,
            screen: Screen::Home,
            preferences,
            search_query: String::new(),
            interactions,
        };

        let task = Task::perform(
            Boot::new(media_preferences, interaction_transport),
            Message::Booted,
        );

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
            Message::CallbackReceived(uri) => {
                if let Status::Ready(boot) = &self.status {
                    if let Err(error) = boot.extensions.deliver_callback(uri) {
                        tracing::warn!(%error, "failed to deliver extension callback");
                    }
                }

                return Task::none();
            }
            Message::Interaction(message) => {
                return self.interactions.update(message).map(Message::Interaction);
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
            (_, Message::Toolbar(toolbar::Message::Navigate(link))) => {
                Action::Navigate(link.into())
            }
            (_, Message::Toolbar(toolbar::Message::QueryChanged(query))) => {
                self.search_query = query;
                Action::None
            }
            (Screen::Search(screen), Message::Toolbar(toolbar::Message::Search)) => Action::run(
                screen
                    .search(self.search_query.clone())
                    .map(Message::Search),
            ),
            (_, Message::Toolbar(toolbar::Message::Search)) => Action::Navigate(Route::Search),
            (_, Message::Navigate(route)) => Action::Navigate(route),
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
                let (screen, task) = search::Search::new(
                    boot.extensions.as_ref(),
                    boot.images.clone(),
                    self.search_query.clone(),
                );
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

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            iced::event::listen_url().map(Message::CallbackReceived),
            self.interactions.subscription().map(Message::Interaction),
        ])
    }

    pub fn view(&self) -> Element<'_, Message> {
        let screen = match &self.status {
            Status::Starting => container(text(t!("common.loading")).body())
                .center(Fill)
                .into(),
            Status::Failed(error) => container(text(error.to_string()).body())
                .center(Fill)
                .into(),
            Status::Ready(_) => match &self.screen {
                Screen::Home => home::view(Route::Search).map(Message::Navigate),
                Screen::Search(screen) => screen.view().map(Message::Search),
                Screen::Series(screen) => screen.view().map(Message::Series),
                Screen::Settings(screen) => screen
                    .view(
                        self.preferences.language(),
                        self.preferences.player_path(),
                        self.preferences.media_proxy(),
                    )
                    .map(Message::Settings),
            },
        };

        let content = if matches!(self.status, Status::Ready(_)) {
            let active = match &self.screen {
                Screen::Home => Some(Link::Home),
                Screen::Search(_) | Screen::Series(_) => None,
                Screen::Settings(screen) => Some(match screen.tab() {
                    SettingsTab::App => Link::Settings,
                    SettingsTab::Extensions => Link::Extensions,
                }),
            };
            main_layout(
                toolbar::toolbar(active, &self.search_query).map(Message::Toolbar),
                screen,
            )
            .into()
        } else {
            screen
        };
        let overlay = match &self.screen {
            Screen::Series(screen) => screen
                .overlay(self.preferences.player_path().is_some())
                .map(|overlay| overlay.map(Message::Series)),
            Screen::Settings(screen) => screen
                .overlay()
                .map(|overlay| overlay.map(Message::Settings)),
            _ => None,
        };

        stack![
            content,
            overlay,
            self.interactions.view().map(Message::Interaction)
        ]
        .into()
    }
}

use std::path::PathBuf;
use std::sync::Arc;

use iced::task::Handle;
use iced::widget::{button, column, container, row, text, text_input};
use iced::{Center, Element, Fill, Task};
use rust_i18n::t;

use crate::components::dialog::overlay;
use crate::components::divider;
use crate::components::styles;
use crate::components::typography::{self, TextExt};
use crate::error::{Error, Result};
use crate::extensions::{CacheLimit, ExtensionId, ExtensionMetadata, Options, Registry};
use crate::fetch::Fetch;
use crate::icons;
use crate::picker;
use crate::theme::PALETTE;
use crate::widgets::extension;

use super::Action;

struct LoadForm {
    file_path: PathBuf,
    metadata: Fetch<Arc<ExtensionMetadata>>,
    cache_dir: PathBuf,
    max_cache_size: u64,
    problem: Option<Error>,
    operation: Option<Handle>,
}

enum Dialog {
    None,
    Info(ExtensionId),
    Load(Box<LoadForm>),
}

pub struct Extensions {
    registry: Arc<Registry>,
    dialog: Dialog,
}

#[derive(Debug, Clone)]
pub enum Message {
    MetadataLoaded(Result<Arc<ExtensionMetadata>>),
    Pick,
    Picked(Option<PathBuf>),
    MaxCacheSizeChanged(u64),
    ConfirmLoad,
    LoadFinished(Result<()>),
    Open(ExtensionId),
    Unload(ExtensionId),
    CloseDialog,
}

impl Extensions {
    pub fn new(registry: Arc<Registry>) -> Self {
        Self {
            registry,
            dialog: Dialog::None,
        }
    }

    pub fn update(&mut self, message: Message) -> Action<Message> {
        match message {
            Message::MetadataLoaded(result) => {
                if let Dialog::Load(form) = &mut self.dialog {
                    form.operation = None;
                    form.metadata = result.into();
                }
                Action::None
            }
            Message::Pick => Action::run(Task::perform(picker::extension(), Message::Picked)),
            Message::Picked(Some(file_path)) => {
                let Options {
                    cache_dir,
                    max_cache_size,
                } = Options::default();
                let inspect = Task::perform(
                    Registry::inspect(file_path.clone()),
                    Message::MetadataLoaded,
                );
                let (inspect, handle) = inspect.abortable();

                self.dialog = Dialog::Load(Box::new(LoadForm {
                    file_path,
                    metadata: Fetch::Loading,
                    cache_dir,
                    max_cache_size: max_cache_size.map_or(0, CacheLimit::megabytes),
                    problem: None,
                    operation: Some(handle.abort_on_drop()),
                }));

                Action::run(inspect)
            }
            Message::Picked(None) => Action::None,
            Message::MaxCacheSizeChanged(input) => {
                if let Dialog::Load(form) = &mut self.dialog {
                    form.max_cache_size = input;
                    form.problem = None;
                }
                Action::None
            }
            Message::ConfirmLoad => {
                let Dialog::Load(form) = &mut self.dialog else {
                    return Action::None;
                };
                if form.operation.is_some() {
                    return Action::None;
                }

                let max_cache_size = (form.max_cache_size > 0)
                    .then(|| CacheLimit::from_megabytes(form.max_cache_size));

                form.problem = None;

                let options = Options {
                    cache_dir: form.cache_dir.clone(),
                    max_cache_size,
                };

                let registry = self.registry.clone();
                let file_path = form.file_path.clone();

                let load = Task::perform(
                    async move { registry.add(file_path, options).await.map(|_| ()) },
                    Message::LoadFinished,
                );
                let (load, handle) = load.abortable();
                form.operation = Some(handle.abort_on_drop());

                Action::run(load)
            }
            Message::LoadFinished(Ok(())) => {
                self.dialog = Dialog::None;
                Action::None
            }
            Message::LoadFinished(Err(error)) => {
                if let Dialog::Load(form) = &mut self.dialog {
                    form.operation = None;
                    form.problem = Some(error);
                }
                Action::None
            }
            Message::Open(id) => {
                self.dialog = Dialog::Info(id);
                Action::None
            }
            Message::Unload(id) => {
                if self.registry.remove(&id).is_none() {
                    tracing::warn!(%id, "asked to unload an extension that is not loaded");
                }
                Action::None
            }
            Message::CloseDialog => {
                self.dialog = Dialog::None;
                Action::None
            }
        }
    }

    pub fn overlay(&self) -> Option<Element<'_, Message>> {
        let dialog = match &self.dialog {
            Dialog::None => return None,
            Dialog::Info(id) => {
                let extension = self.registry.get(id)?;

                Element::from(extension::info_dialog(&extension, Message::CloseDialog))
            }
            Dialog::Load(form) => load_dialog(form),
        };

        Some(overlay(dialog, Message::CloseDialog))
    }

    pub fn view(&self) -> Element<'_, Message> {
        let extensions = self.registry.values();
        let content = if extensions.is_empty() {
            Element::from(
                row![
                    icons::blocks().size(24).color(PALETTE.border),
                    text(t!("settings.extensions.status_idle"))
                        .body()
                        .color(PALETTE.text_muted),
                ]
                .align_y(Center)
                .spacing(8),
            )
        } else {
            Element::from(
                column(extensions.iter().map(|extension| {
                    extension::card(
                        extension,
                        Message::Open(extension.id.clone()),
                        Message::Unload(extension.id.clone()),
                    )
                }))
                .spacing(4),
            )
        };

        column![
            row![
                text(t!("settings.extensions.loaded_label"))
                    .section()
                    .width(Fill),
                button(text(t!("settings.extensions.load")).label())
                    .on_press(Message::Pick)
                    .style(styles::outline_button)
            ]
            .align_y(Center)
            .spacing(16),
            divider(),
            content
        ]
        .spacing(16)
        .into()
    }
}

fn load_dialog(form: &LoadForm) -> Element<'_, Message> {
    let hint = match &form.problem {
        Some(problem) => text(problem.to_string()).hint().color(PALETTE.text),
        None => text(t!(
            "settings.extensions.options.max_cache_size_hint",
            max = CacheLimit::MAX.to_string()
        ))
        .hint(),
    };

    let size_field = column![
        text_input("", form.max_cache_size.to_string())
            .on_input(move |value| {
                Message::MaxCacheSizeChanged(
                    value
                        .parse()
                        .ok()
                        .filter(|value| *value <= CacheLimit::MAX.megabytes())
                        .unwrap_or(form.max_cache_size),
                )
            })
            .size(typography::LABEL)
            .padding([8, 12])
            .style(styles::text_field)
            .width(Fill),
        hint,
    ]
    .spacing(4)
    .into();

    let title = match form
        .metadata
        .loaded()
        .and_then(|metadata| metadata.name.clone())
    {
        Some(title) => text::Fragment::Owned(title),
        None => t!("settings.extensions.meta.fallback_title"),
    };

    let panel = match &form.metadata {
        Fetch::Loading => container(text(t!("common.loading")).body())
            .center(Fill)
            .into(),
        Fetch::Failed(error) => container(text(error.to_string()).body())
            .center(Fill)
            .into(),
        Fetch::Loaded(metadata) => extension::meta_panel(&form.file_path, metadata.as_ref()),
    };

    extension::load_dialog(
        title,
        panel,
        text_input(
            t!("settings.extensions.options.cache_dir_placeholder"),
            form.cache_dir.to_string_lossy(),
        )
        .size(typography::LABEL)
        .padding([8, 12])
        .style(styles::text_field)
        .into(),
        size_field,
        (form.operation.is_none() && form.metadata.loaded().is_some())
            .then_some(Message::ConfirmLoad),
        Message::CloseDialog,
    )
    .into()
}

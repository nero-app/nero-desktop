use std::path::Path;

use iced::widget::{button, column, row, space, text, Container};
use iced::{Center, Element, Fill};
use nero_extensions::Extension;
use rust_i18n::t;

use crate::components::dialog::dialog;
use crate::components::styles;
use crate::components::typography::TextExt;
use crate::components::vertical_divider;
use crate::extensions::{ExtensionMetadata, LoadedExtension};
use crate::icons;
use crate::theme::PALETTE;

pub fn title(extension: &LoadedExtension) -> String {
    if let Some(name) = &extension.extension.metadata().name {
        return name.clone();
    }

    extension
        .file_path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default()
}

fn labelled<'a, Message: 'a>(
    label: impl text::IntoFragment<'a>,
    value: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    column![text(label).label().color(PALETTE.text_muted), value.into()]
        .spacing(4)
        .into()
}

pub fn card<Message>(
    extension: &LoadedExtension,
    on_open: Message,
    on_unload: Message,
) -> Element<'static, Message>
where
    Message: Clone + 'static,
{
    let metadata = extension.extension.metadata();

    let subtitle = [
        metadata
            .version
            .as_ref()
            .map(|version| format!("v{version}")),
        metadata.authors.as_ref().map(ToString::to_string),
        metadata.licenses.as_ref().map(ToString::to_string),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();

    let subtitle = (!subtitle.is_empty()).then(|| text(subtitle.join(" • ")).hint().truncate());
    let labels = column![text(title(extension)).body().truncate(), subtitle].width(Fill);

    row![
        button(labels)
            .on_press(on_open)
            .width(Fill)
            .padding([10, 12])
            .style(styles::card_button),
        button(icons::x().size(16).color(PALETTE.text_muted))
            .on_press(on_unload)
            .padding(8)
            .style(styles::link_button),
    ]
    .align_y(Center)
    .spacing(8)
    .into()
}

pub fn meta_panel<Message: 'static>(
    file_path: &Path,
    metadata: &ExtensionMetadata,
) -> Element<'static, Message> {
    let fields = [
        (
            t!("settings.extensions.meta.version"),
            metadata.version.as_ref().map(ToString::to_string),
        ),
        (
            t!("settings.extensions.meta.revision"),
            metadata.revision.as_ref().map(ToString::to_string),
        ),
        (
            t!("settings.extensions.meta.authors"),
            metadata.authors.as_ref().map(ToString::to_string),
        ),
        (
            t!("settings.extensions.meta.licenses"),
            metadata.licenses.as_ref().map(ToString::to_string),
        ),
        (
            t!("settings.extensions.meta.homepage"),
            metadata.homepage.as_ref().map(ToString::to_string),
        ),
        (
            t!("settings.extensions.meta.source"),
            metadata.source.as_ref().map(ToString::to_string),
        ),
    ];

    let dependencies = metadata.dependencies.as_ref().map(|dependencies| {
        dependencies
            .version_info()
            .packages
            .iter()
            .map(|package| package.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    });

    let description = metadata
        .description
        .as_ref()
        .map(|description| text(description.to_string()).body());
    let fields = fields
        .into_iter()
        .filter_map(|(label, value)| Some(labelled(label, text(value?).body())));
    let dependencies = dependencies.map(|dependencies| {
        labelled(
            t!("settings.extensions.meta.dependencies"),
            text(dependencies).body(),
        )
    });
    let items = column![
        text(file_path.to_string_lossy().into_owned()).hint(),
        description
    ]
    .extend(fields)
    .push(dependencies)
    .spacing(12)
    .padding([0, 16]);

    iced::widget::scrollable(items)
        .width(Fill)
        .height(Fill)
        .into()
}

fn panels<'a, Message: 'a>(
    left: Element<'a, Message>,
    right: Element<'a, Message>,
) -> Element<'a, Message> {
    row![left, vertical_divider(), right]
        .spacing(16)
        .padding([16, 0])
        .into()
}

pub fn info_dialog<Message>(
    extension: &LoadedExtension,
    on_close: Message,
) -> Container<'static, Message>
where
    Message: Clone + 'static,
{
    let metadata = extension.extension.metadata();

    let limit = match extension.options.max_cache_size {
        Some(limit) => limit.to_string(),
        None => t!("common.unlimited").to_string(),
    };

    let panel = column![
        text(t!("settings.extensions.options.title")).section(),
        labelled(
            t!("settings.extensions.options.cache_dir"),
            text(extension.options.cache_dir.to_string_lossy().into_owned()).hint()
        ),
        labelled(
            t!("settings.extensions.options.max_cache_size"),
            text(limit).body()
        ),
    ]
    .spacing(16)
    .padding([0, 16])
    .width(Fill);

    dialog(
        title(extension),
        panels(meta_panel(&extension.file_path, &metadata), panel.into()),
        on_close,
    )
}

pub fn load_dialog<'a, Message>(
    title: impl text::IntoFragment<'a>,
    metadata: Element<'a, Message>,
    cache_dir: Element<'a, Message>,
    max_cache_size: Element<'a, Message>,
    on_confirm: Option<Message>,
    on_close: Message,
) -> Container<'a, Message>
where
    Message: Clone + 'static,
{
    let options = column![
        text(t!("settings.extensions.options.title")).section(),
        labelled(t!("settings.extensions.options.cache_dir"), cache_dir),
        labelled(
            t!("settings.extensions.options.max_cache_size"),
            max_cache_size
        ),
        space().height(Fill),
        button(
            text(t!("settings.extensions.load"))
                .body()
                .center()
                .width(Fill)
        )
        .on_press_maybe(on_confirm)
        .width(Fill)
        .padding(10)
        .style(styles::primary_button),
    ]
    .spacing(16)
    .padding([0, 16])
    .width(Fill)
    .height(Fill);

    dialog(title, panels(metadata, options.into()), on_close)
}

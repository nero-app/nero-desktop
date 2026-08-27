use std::borrow::Cow;
use std::path::{Path, PathBuf};

use iced::widget::{button, column, container, pick_list, row, text, toggler};
use iced::{Center, Element, Fill, Task};
use rust_i18n::t;

use crate::components::divider;
use crate::components::styles;
use crate::components::typography::{self, TextExt};
use crate::i18n::Language;
use crate::media::default_torrent_dir;
use crate::picker;
use crate::preferences::{MediaProxyPreferences, PreferenceAction};

use super::Action;
use crate::theme::PALETTE;

#[derive(Debug, Clone)]
pub enum Message {
    LanguageSelected(Language),
    PickPlayer,
    PlayerPicked(Option<PathBuf>),
    ToggleTorrent(bool),
    PickOutputFolder,
    OutputFolderPicked(Option<PathBuf>),
    ResetOutputFolder,
}

pub fn update(message: Message) -> Action<Message> {
    match message {
        Message::LanguageSelected(language) => Action::emit(PreferenceAction::Language(language)),
        Message::PickPlayer => Action::run(Task::perform(picker::player(), Message::PlayerPicked)),
        Message::PlayerPicked(Some(path)) => Action::emit(PreferenceAction::PlayerPath(path)),
        Message::PlayerPicked(None) => Action::None,
        Message::ToggleTorrent(enabled) => Action::emit(PreferenceAction::TorrentEnabled(enabled)),
        Message::PickOutputFolder => {
            Action::run(Task::perform(picker::folder(), Message::OutputFolderPicked))
        }
        Message::OutputFolderPicked(Some(folder)) => {
            Action::emit(PreferenceAction::TorrentOutputFolder(Some(folder)))
        }
        Message::OutputFolderPicked(None) => Action::None,
        Message::ResetOutputFolder => Action::emit(PreferenceAction::TorrentOutputFolder(None)),
    }
}

pub fn view<'a>(
    language: Language,
    player_path: Option<&'a Path>,
    media_proxy: &'a MediaProxyPreferences,
) -> Element<'a, Message> {
    column![
        language_setting(language),
        divider(),
        player(player_path),
        divider(),
        torrents(media_proxy)
    ]
    .spacing(16)
    .into()
}

fn language_setting<'a>(language: Language) -> Element<'a, Message> {
    let select = pick_list(Some(language), Language::ALL, ToString::to_string)
        .on_select(Message::LanguageSelected)
        .text_size(typography::LABEL)
        .padding([8, 12])
        .style(styles::select);

    row![
        column![
            text(t!("settings.app.language.title")).section(),
            text(t!("settings.app.language.description"))
                .label()
                .color(PALETTE.text_muted),
        ]
        .spacing(4)
        .width(Fill),
        select
    ]
    .align_y(Center)
    .spacing(16)
    .into()
}

fn player(player_path: Option<&Path>) -> Element<'_, Message> {
    let path = match player_path {
        Some(path) => path.to_string_lossy(),
        None => t!("settings.app.player.not_selected"),
    };

    let change = button(text(t!("common.change")).label())
        .on_press(Message::PickPlayer)
        .style(styles::outline_button);

    let value = container(
        text(path)
            .body()
            .color(PALETTE.text_muted)
            .wrapping(text::Wrapping::None),
    )
    .width(Fill)
    .clip(true);

    column![
        column![
            text(t!("settings.app.player.title")).section(),
            text(t!("settings.app.player.description"))
                .label()
                .color(PALETTE.text_muted),
        ]
        .spacing(4),
        row![value, change].align_y(Center).spacing(16)
    ]
    .spacing(16)
    .into()
}

fn torrents(preferences: &MediaProxyPreferences) -> Element<'_, Message> {
    let header = row![
        column![
            text(t!("settings.app.torrent.title")).section(),
            text(t!("settings.app.torrent.description"))
                .label()
                .color(PALETTE.text_muted),
        ]
        .spacing(4)
        .width(Fill),
        toggler(preferences.torrent_enabled)
            .on_toggle(Message::ToggleTorrent)
            .size(24.0)
            .style(styles::toggle),
    ]
    .align_y(Center)
    .spacing(16);

    let folder = preferences
        .torrent_enabled
        .then(|| output_folder(preferences));

    column![header, folder].spacing(16).into()
}

fn output_folder(preferences: &MediaProxyPreferences) -> Element<'_, Message> {
    let folder = match &preferences.torrent_output_folder {
        Some(folder) => folder.to_string_lossy(),
        None => Cow::Owned(default_torrent_dir().to_string_lossy().into_owned()),
    };

    let value = container(
        column![
            text(t!("settings.app.torrent.output_folder_label")).heading(),
            text(folder)
                .label()
                .color(PALETTE.text_muted)
                .wrapping(text::Wrapping::None),
        ]
        .width(Fill),
    )
    .width(Fill)
    .clip(true);

    let change = button(text(t!("common.change")).label())
        .on_press(Message::PickOutputFolder)
        .style(styles::outline_button);

    let reset = preferences.torrent_output_folder.is_some().then(|| {
        button(text(t!("common.reset")).label())
            .on_press(Message::ResetOutputFolder)
            .style(styles::outline_button)
    });

    row![value, row![change, reset].spacing(8)]
        .align_y(Center)
        .spacing(16)
        .into()
}

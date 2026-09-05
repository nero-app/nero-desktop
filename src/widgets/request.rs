use std::collections::BTreeSet;

use iced::widget::{
    button, checkbox, column, container, radio, row, scrollable, space, text, text_input,
};
use iced::{Center, Element, Fill, Length};
use nero_extensions::{SelectionRequest, Severity, TextRequest};
use rust_i18n::t;

use crate::components::dialog as modal;
use crate::components::styles;
use crate::components::typography::{self, TextExt};
use crate::interactions::request::{Action, Request, RequestId};
use crate::theme::PALETTE;

pub fn view(request: &Request) -> Element<'_, Action> {
    let can_submit = request.can_submit();
    match request {
        Request::Text {
            id,
            input,
            request,
            value,
            ..
        } => text_request(id, input, request, value, can_submit),
        Request::Selection {
            id,
            request,
            selected,
            ..
        } => selection_request(id, request, selected, can_submit),
    }
}

fn text_request<'a>(
    id: &'a RequestId,
    input: &'a iced::widget::Id,
    request: &'a TextRequest,
    value: &'a str,
    can_submit: bool,
) -> Element<'a, Action> {
    let input = text_input(request.placeholder.as_deref().unwrap_or_default(), value)
        .id(input.clone())
        .on_input(Action::TextChanged)
        .on_submit_maybe(can_submit.then_some(Action::Submit))
        .secure(request.sensitive)
        .size(typography::BODY)
        .padding([10, 12])
        .style(styles::text_field);
    let reference = request.reference.as_ref().map(|reference| {
        column![
            text(t!("interactions.reference")).label_strong(),
            text(reference).label().color(PALETTE.text_muted),
        ]
        .spacing(2)
    });
    let content = column![
        request
            .description
            .as_ref()
            .map(|description| text(description).body()),
        input,
        reference
    ]
    .spacing(16);

    dialog(
        id,
        &request.title,
        Severity::Info,
        content.into(),
        can_submit,
    )
}

fn selection_request<'a>(
    id: &'a RequestId,
    request: &'a SelectionRequest,
    selected: &'a BTreeSet<u32>,
    can_submit: bool,
) -> Element<'a, Action> {
    let selected_single = selected.iter().next().copied();
    let choices = column(request.choices.iter().enumerate().map(|(index, choice)| {
        let index = index as u32;
        let control: Element<'_, Action> = if request.allow_multiple {
            checkbox(selected.contains(&index))
                .label(choice.label.clone())
                .on_toggle(move |_| Action::ChoiceToggled(index))
                .into()
        } else {
            radio(choice.label.clone(), index, selected_single, move |index| {
                Action::ChoiceSelected(index)
            })
            .into()
        };

        container(
            column![
                control,
                choice.detail.as_ref().map(|detail| {
                    container(text(detail).label().color(PALETTE.text_muted))
                        .padding(iced::padding::left(24))
                }),
            ]
            .spacing(4),
        )
        .padding([6, 0])
        .into()
    }))
    .spacing(2);
    let content = column![
        request
            .description
            .as_ref()
            .map(|description| text(description).body()),
        scrollable(choices).height(Length::Fill.max(260.0)),
    ]
    .spacing(12);

    dialog(
        id,
        &request.title,
        request.severity,
        content.into(),
        can_submit,
    )
}

fn dialog<'a>(
    id: &'a RequestId,
    title: &'a str,
    severity: Severity,
    body: Element<'a, Action>,
    can_submit: bool,
) -> Element<'a, Action> {
    let header = column![
        text(title).heading(),
        text(t!(
            "interactions.requested_by",
            extension = id.extension_id().as_ref()
        ))
        .hint(),
    ]
    .spacing(4);
    let actions = row![
        space().width(Fill),
        button(text(t!("common.cancel")).label())
            .on_press(Action::Dismiss)
            .padding([8, 14])
            .style(styles::outline_button),
        button(text(t!("common.continue")).label())
            .on_press_maybe(can_submit.then_some(Action::Submit))
            .padding([8, 14])
            .style(styles::primary_button),
    ]
    .align_y(Center)
    .spacing(8);
    let border_color = match severity {
        Severity::Info => PALETTE.border,
        Severity::Warning => iced::Color::from_rgb8(217, 119, 6),
        Severity::Error => iced::Color::from_rgb8(220, 38, 38),
    };
    let content = container(column![header, body, actions].spacing(20))
        .width(Length::Fill.max(560.0))
        .padding(24)
        .style(move |_| container::Style {
            background: Some(PALETTE.surface.into()),
            border: iced::Border {
                color: border_color,
                width: 1.0,
                radius: 8.0.into(),
            },
            ..container::Style::default()
        });

    modal::overlay(content, Action::Dismiss)
}

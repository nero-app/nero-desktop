use std::collections::{BTreeSet, VecDeque};

use iced::advanced::widget::{operate, operation::focusable};
use iced::widget::{keyed_column, operation, space};
use iced::{Element, Fill, Task};
use nero_extensions::{SelectionRequest, TextRequest};
use tokio::sync::oneshot;

use crate::extensions::ExtensionId;

#[derive(Clone, PartialEq, Eq)]
pub struct RequestId {
    key: u64,
    extension_id: ExtensionId,
}

impl RequestId {
    pub fn new(key: u64, extension_id: ExtensionId) -> Self {
        Self { key, extension_id }
    }

    pub fn extension_id(&self) -> &ExtensionId {
        &self.extension_id
    }
}

#[derive(Clone)]
pub struct Message {
    id: RequestId,
    action: Action,
}

#[derive(Clone)]
pub enum Action {
    TextChanged(String),
    ChoiceSelected(u32),
    ChoiceToggled(u32),
    Submit,
    Dismiss,
}

pub enum Event {
    Text {
        id: RequestId,
        request: TextRequest,
        respond: oneshot::Sender<Option<String>>,
    },
    Selection {
        id: RequestId,
        request: SelectionRequest,
        respond: oneshot::Sender<Option<Vec<u32>>>,
    },
    Cancelled(RequestId),
}

pub enum Request {
    Text {
        id: RequestId,
        input: iced::widget::Id,
        request: TextRequest,
        value: String,
        respond: oneshot::Sender<Option<String>>,
    },
    Selection {
        id: RequestId,
        request: SelectionRequest,
        selected: BTreeSet<u32>,
        respond: oneshot::Sender<Option<Vec<u32>>>,
    },
}

impl Request {
    pub fn can_submit(&self) -> bool {
        match self {
            Request::Text { .. } => true,
            Request::Selection { selected, .. } => !selected.is_empty(),
        }
    }

    fn id(&self) -> &RequestId {
        match self {
            Request::Text { id, .. } | Request::Selection { id, .. } => id,
        }
    }
}

#[derive(Default)]
pub struct Requests {
    pending: VecDeque<Request>,
    active: Option<Request>,
}

impl Requests {
    pub fn receive(&mut self, event: Event) -> Task<Message> {
        match event {
            Event::Text {
                id,
                request,
                respond,
            } => self.push_text(id, request, respond),
            Event::Selection {
                id,
                request,
                respond,
            } => self.push_selection(id, request, respond),
            Event::Cancelled(id) => self.cancel(&id),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        if !self
            .active
            .as_ref()
            .is_some_and(|request| request.id() == &message.id)
        {
            return Task::none();
        }

        match message.action {
            Action::TextChanged(value) => {
                if let Some(Request::Text { value: current, .. }) = &mut self.active {
                    *current = value;
                }
            }
            Action::ChoiceSelected(index) => {
                if let Some(Request::Selection {
                    request, selected, ..
                }) = &mut self.active
                {
                    if request.choices.get(index as usize).is_some() {
                        selected.clear();
                        selected.insert(index);
                    }
                }
            }
            Action::ChoiceToggled(index) => {
                if let Some(Request::Selection {
                    request, selected, ..
                }) = &mut self.active
                {
                    if request.choices.get(index as usize).is_some() && !selected.remove(&index) {
                        selected.insert(index);
                    }
                }
            }
            Action::Submit => return self.answer(true),
            Action::Dismiss => return self.answer(false),
        }

        Task::none()
    }

    fn push_text(
        &mut self,
        id: RequestId,
        mut request: TextRequest,
        respond: oneshot::Sender<Option<String>>,
    ) -> Task<Message> {
        let value = request.initial_value.take().unwrap_or_default();
        self.push(Request::Text {
            id,
            input: iced::widget::Id::unique(),
            request,
            value,
            respond,
        })
    }

    fn push_selection(
        &mut self,
        id: RequestId,
        request: SelectionRequest,
        respond: oneshot::Sender<Option<Vec<u32>>>,
    ) -> Task<Message> {
        let mut selected = request
            .choices
            .iter()
            .enumerate()
            .filter_map(|(index, choice)| choice.preselected.then_some(index as u32))
            .collect::<BTreeSet<_>>();

        if !request.allow_multiple {
            selected = selected.into_iter().next().into_iter().collect();
        }

        self.push(Request::Selection {
            id,
            request,
            selected,
            respond,
        })
    }

    fn cancel(&mut self, id: &RequestId) -> Task<Message> {
        self.pending.retain(|request| request.id() != id);
        if self
            .active
            .as_ref()
            .is_some_and(|request| request.id() == id)
        {
            self.active = self.pending.pop_front();
            return self.focus_active();
        }

        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        match self.active.as_ref() {
            Some(request) => {
                let view = crate::widgets::request::view(request).map(|action| Message {
                    id: request.id().clone(),
                    action,
                });
                keyed_column([(request.id().key, view)])
                    .width(Fill)
                    .height(Fill)
                    .into()
            }
            None => space().width(Fill).height(Fill).into(),
        }
    }

    fn push(&mut self, request: Request) -> Task<Message> {
        if self.active.is_none() {
            self.active = Some(request);
            self.focus_active()
        } else {
            self.pending.push_back(request);
            Task::none()
        }
    }

    fn answer(&mut self, submit: bool) -> Task<Message> {
        if !self
            .active
            .as_ref()
            .is_some_and(|request| !submit || request.can_submit())
        {
            return Task::none();
        }

        if let Some(request) = self.active.take() {
            match request {
                Request::Text { value, respond, .. } => {
                    let _ = respond.send(submit.then_some(value));
                }
                Request::Selection {
                    selected, respond, ..
                } => {
                    let answer = submit.then(|| selected.into_iter().collect());
                    let _ = respond.send(answer);
                }
            }
        }

        self.active = self.pending.pop_front();
        self.focus_active()
    }

    fn focus_active(&self) -> Task<Message> {
        match self.active.as_ref() {
            Some(Request::Text { input, .. }) => operation::focus(input.clone()),
            Some(Request::Selection { .. }) => operate(focusable::unfocus()),
            None => Task::none(),
        }
    }
}

use std::collections::VecDeque;
use std::time::Duration;

use iced::task::Handle;
use iced::{Color, Element, Task};
use nero_extensions::Severity;
use tokio::sync::OwnedSemaphorePermit;

use crate::extensions::ExtensionId;
use crate::theme::PALETTE;

const MAX_VISIBLE_NOTIFICATIONS: usize = 4;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct NotificationId(u64);

#[derive(Clone)]
pub enum Message {
    Dismiss(NotificationId),
    AnimationFinished(NotificationId),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Entering,
    Visible,
    Exiting,
}

pub struct Event {
    pub extension_id: ExtensionId,
    pub severity: Severity,
    pub message: String,
    pub detail: Option<String>,
    pub permit: OwnedSemaphorePermit,
}

pub struct Notification {
    pub id: NotificationId,
    pub extension_id: ExtensionId,
    pub severity: Severity,
    pub message: String,
    pub detail: Option<String>,
    _permit: OwnedSemaphorePermit,
    phase: Phase,
    timeout: Option<Handle>,
}

#[derive(Default)]
pub struct Notifications {
    items: VecDeque<Notification>,
    next_id: u64,
}

impl Notifications {
    pub fn receive(&mut self, event: Event) -> Task<Message> {
        let Event {
            extension_id,
            severity,
            message,
            detail,
            permit,
        } = event;
        let id = NotificationId(self.next_id);
        self.next_id = self.next_id.wrapping_add(1);

        while self.items.len() >= MAX_VISIBLE_NOTIFICATIONS {
            self.items.pop_front();
        }

        let timeout = match severity {
            Severity::Info => Duration::from_secs(6),
            Severity::Warning | Severity::Error => Duration::from_secs(10),
        };
        let dismissal = Task::perform(
            async move { tokio::time::sleep(timeout).await },
            move |_| Message::Dismiss(id),
        );
        let (dismissal, handle) = dismissal.abortable();
        self.items.push_back(Notification {
            id,
            extension_id,
            severity,
            message,
            detail,
            _permit: permit,
            phase: Phase::Entering,
            timeout: Some(handle.abort_on_drop()),
        });

        dismissal
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Dismiss(id) => {
                let Some(index) = self
                    .items
                    .iter()
                    .position(|notification| notification.id == id)
                else {
                    return;
                };

                match self.items[index].phase {
                    Phase::Entering => {
                        self.items.remove(index);
                    }
                    Phase::Visible => {
                        let notification = &mut self.items[index];
                        notification.timeout = None;
                        notification.phase = Phase::Exiting;
                    }
                    Phase::Exiting => {}
                }
            }
            Message::AnimationFinished(id) => {
                let Some(index) = self
                    .items
                    .iter()
                    .position(|notification| notification.id == id)
                else {
                    return;
                };

                match self.items[index].phase {
                    Phase::Entering => self.items[index].phase = Phase::Visible,
                    Phase::Visible => {}
                    Phase::Exiting => {
                        self.items.remove(index);
                    }
                }
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        crate::components::toast::group(self.items.iter().rev().map(|notification| {
            crate::components::toast::toast(notification.id, || {
                crate::widgets::notification::view(notification)
            })
            .border_color(match notification.severity {
                Severity::Info => PALETTE.border,
                Severity::Warning => Color::from_rgb8(217, 119, 6),
                Severity::Error => Color::from_rgb8(220, 38, 38),
            })
            .visible(notification.phase != Phase::Exiting)
            .on_finish(Message::AnimationFinished(notification.id))
        }))
    }
}

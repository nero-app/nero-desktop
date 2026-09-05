use iced::Element;
use nero_extensions::Cancel;

use crate::extensions::ExtensionId;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ProgressId {
    key: u64,
    extension_id: ExtensionId,
}

impl ProgressId {
    pub fn new(key: u64, extension_id: ExtensionId) -> Self {
        Self { key, extension_id }
    }

    pub fn extension_id(&self) -> &ExtensionId {
        &self.extension_id
    }
}

pub enum Event {
    Started {
        id: ProgressId,
        title: String,
        cancellable: bool,
        cancel: Cancel,
    },
    Reported {
        id: ProgressId,
        message: Option<String>,
        percent: Option<u8>,
    },
    Finished(ProgressId),
}

#[derive(Clone)]
pub enum Message {
    Cancel(ProgressId),
    AnimationFinished(ProgressId),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Entering,
    Visible,
    Exiting,
}

pub struct ProgressEntry {
    id: ProgressId,
    pub title: String,
    pub message: Option<String>,
    pub percent: Option<u8>,
    pub cancellable: bool,
    pub cancelling: bool,
    phase: Phase,
    cancel: Cancel,
}

#[derive(Default)]
pub struct Progresses {
    entries: Vec<ProgressEntry>,
}

impl Progresses {
    pub fn receive(&mut self, event: Event) {
        match event {
            Event::Started {
                id,
                title,
                cancellable,
                cancel,
            } => {
                self.start(id, title, cancellable, cancel);
            }
            Event::Reported {
                id,
                message,
                percent,
            } => {
                self.report(&id, message, percent);
            }
            Event::Finished(id) => self.finish(&id),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Cancel(id) => {
                if let Some(progress) = self.entries.iter_mut().find(|progress| progress.id == id) {
                    if progress.phase != Phase::Exiting
                        && progress.cancellable
                        && !progress.cancelling
                    {
                        progress.cancelling = true;
                        progress.cancel.cancel();
                    }
                }
            }
            Message::AnimationFinished(id) => {
                let Some(index) = self.entries.iter().position(|progress| progress.id == id) else {
                    return;
                };

                match self.entries[index].phase {
                    Phase::Entering => self.entries[index].phase = Phase::Visible,
                    Phase::Visible => {}
                    Phase::Exiting => {
                        self.entries.remove(index);
                    }
                }
            }
        }
    }

    fn start(&mut self, id: ProgressId, title: String, cancellable: bool, cancel: Cancel) {
        self.entries.push(ProgressEntry {
            id,
            title,
            message: None,
            percent: None,
            cancellable,
            cancelling: false,
            phase: Phase::Entering,
            cancel,
        });
    }

    fn report(&mut self, id: &ProgressId, message: Option<String>, percent: Option<u8>) {
        let Some(progress) = self.entries.iter_mut().find(|progress| progress.id == *id) else {
            return;
        };

        if let Some(message) = message {
            progress.message = Some(message);
        }
        if let Some(percent) = percent {
            progress.percent = Some(percent.min(100));
        }
    }

    fn finish(&mut self, id: &ProgressId) {
        let Some(index) = self.entries.iter().position(|progress| progress.id == *id) else {
            return;
        };

        match self.entries[index].phase {
            Phase::Entering => {
                self.entries.remove(index);
            }
            Phase::Visible => self.entries[index].phase = Phase::Exiting,
            Phase::Exiting => {}
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        crate::components::toast::group(self.entries.iter().map(|progress| {
            crate::components::toast::toast(progress.id.key, || {
                crate::widgets::progress::view(&progress.id, progress)
            })
            .visible(progress.phase != Phase::Exiting)
            .on_finish(Message::AnimationFinished(progress.id.clone()))
        }))
    }
}

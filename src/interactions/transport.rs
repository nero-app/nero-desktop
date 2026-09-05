use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

use iced::futures::channel::mpsc;

use super::Event;

#[derive(Default)]
pub struct Transport {
    sender: Mutex<Option<mpsc::UnboundedSender<Event>>>,
    next_key: AtomicU64,
}

impl Transport {
    pub fn connect(&self, sender: mpsc::UnboundedSender<Event>) {
        if let Ok(mut current) = self.sender.lock() {
            *current = Some(sender);
        }
    }

    pub fn send(&self, event: Event) -> bool {
        let Ok(sender) = self.sender.lock() else {
            return false;
        };
        let Some(sender) = sender.as_ref() else {
            return false;
        };

        sender.unbounded_send(event).is_ok()
    }

    pub fn next_key(&self) -> u64 {
        self.next_key.fetch_add(1, Ordering::Relaxed)
    }
}

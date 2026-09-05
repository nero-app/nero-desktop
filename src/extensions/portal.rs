use std::sync::{Arc, Mutex};

use nero_extensions::{
    CallbackTransport, Cancel, Delivery, ListenError, ProgressHandle, SelectionRequest, Severity,
    TextRequest,
};
use tokio::sync::{oneshot, OwnedSemaphorePermit, Semaphore};
use url::Url;

use crate::error::{Error, Result};
use crate::interactions::{self, ProgressId, RequestId};

use super::ExtensionId;

type ResponseSender<T> = oneshot::Sender<Option<T>>;
type RequestEvent = interactions::request::Event;

pub struct Interaction {
    extension_id: ExtensionId,
    transport: Arc<interactions::Transport>,
    request_slots: Arc<Semaphore>,
    notification_slots: Arc<Semaphore>,
    progress_slots: Arc<Semaphore>,
}

impl Interaction {
    const MAX_PENDING_REQUESTS: usize = 8;
    const MAX_LIVE_NOTIFICATIONS: usize = 4;
    const MAX_LIVE_PROGRESSES: usize = 4;

    pub fn new(extension_id: ExtensionId, transport: Arc<interactions::Transport>) -> Self {
        Self {
            extension_id,
            transport,
            request_slots: Arc::new(Semaphore::new(Self::MAX_PENDING_REQUESTS)),
            notification_slots: Arc::new(Semaphore::new(Self::MAX_LIVE_NOTIFICATIONS)),
            progress_slots: Arc::new(Semaphore::new(Self::MAX_LIVE_PROGRESSES)),
        }
    }

    async fn request<T, F>(&self, event: F) -> Option<T>
    where
        T: Send,
        F: FnOnce(RequestId, ResponseSender<T>) -> RequestEvent + Send,
    {
        let permit = self.request_slots.clone().try_acquire_owned().ok()?;
        let id = RequestId::new(self.transport.next_key(), self.extension_id.clone());
        let (respond, response) = oneshot::channel();

        if !self
            .transport
            .send(interactions::Event::Request(event(id.clone(), respond)))
        {
            return None;
        }

        let mut guard = RequestGuard {
            id,
            transport: self.transport.clone(),
            _permit: permit,
            completed: false,
        };
        let response = response.await.unwrap_or(None);
        guard.completed = true;
        response
    }
}

struct RequestGuard {
    id: RequestId,
    transport: Arc<interactions::Transport>,
    _permit: OwnedSemaphorePermit,
    completed: bool,
}

impl Drop for RequestGuard {
    fn drop(&mut self) {
        if !self.completed {
            self.transport.send(interactions::Event::Request(
                interactions::request::Event::Cancelled(self.id.clone()),
            ));
        }
    }
}

struct Progress {
    id: ProgressId,
    transport: Arc<interactions::Transport>,
    _permit: OwnedSemaphorePermit,
}

impl ProgressHandle for Progress {
    fn report(&self, message: Option<String>, percent: Option<u8>) {
        self.transport.send(interactions::Event::Progress(
            interactions::progress::Event::Reported {
                id: self.id.clone(),
                message,
                percent,
            },
        ));
    }
}

impl Drop for Progress {
    fn drop(&mut self) {
        self.transport.send(interactions::Event::Progress(
            interactions::progress::Event::Finished(self.id.clone()),
        ));
    }
}

#[async_trait::async_trait]
impl nero_extensions::Interaction for Interaction {
    async fn request_text(&self, request: TextRequest) -> Option<String> {
        self.request(move |id, respond| interactions::request::Event::Text {
            id,
            request,
            respond,
        })
        .await
    }

    async fn request_selection(&self, request: SelectionRequest) -> Option<Vec<u32>> {
        self.request(move |id, respond| interactions::request::Event::Selection {
            id,
            request,
            respond,
        })
        .await
    }

    fn notify(&self, severity: Severity, message: String, detail: Option<String>) {
        let Ok(permit) = self.notification_slots.clone().try_acquire_owned() else {
            return;
        };

        self.transport.send(interactions::Event::Notification(
            interactions::notification::Event {
                extension_id: self.extension_id.clone(),
                severity,
                message,
                detail,
                permit,
            },
        ));
    }

    fn begin_progress(
        &self,
        title: String,
        cancellable: bool,
        cancel: Cancel,
    ) -> Option<Box<dyn ProgressHandle>> {
        let permit = self.progress_slots.clone().try_acquire_owned().ok()?;
        let id = ProgressId::new(self.transport.next_key(), self.extension_id.clone());
        let sent = self.transport.send(interactions::Event::Progress(
            interactions::progress::Event::Started {
                id: id.clone(),
                title,
                cancellable,
                cancel,
            },
        ));
        if !sent {
            return None;
        }

        Some(Box::new(Progress {
            id,
            transport: self.transport.clone(),
            _permit: permit,
        }))
    }
}

pub struct Opener;

#[async_trait::async_trait]
impl nero_extensions::Opener for Opener {
    async fn open(&self, uri: &str) -> bool {
        let Ok(uri) = Url::parse(uri) else {
            return false;
        };

        if !matches!(uri.scheme(), "http" | "https") || uri.host_str().is_none() {
            return false;
        }

        open::that_detached(uri.as_str()).is_ok()
    }
}

pub struct Callbacks {
    pub(super) address: String,
    pending: Mutex<Option<Delivery>>,
}

impl Callbacks {
    pub fn new(id: &ExtensionId) -> Self {
        let mut address = Url::parse("nero://callback/").expect("valid callback URL");
        address
            .path_segments_mut()
            .expect("callback URL supports path segments")
            .push(id.as_ref());

        Self {
            address: address.to_string(),
            pending: Mutex::new(None),
        }
    }

    pub fn deliver(&self, uri: String) -> Result<()> {
        let delivery = self
            .pending
            .lock()
            .map_err(|_| Error::CallbackLockPoisoned)?
            .take()
            .filter(|delivery| !delivery.is_abandoned())
            .ok_or_else(|| Error::CallbackNotPending(self.address.clone()))?;

        delivery.deliver(uri);
        Ok(())
    }
}

impl CallbackTransport for Callbacks {
    fn register(&self, delivery: Delivery) -> std::result::Result<String, ListenError> {
        let mut pending = self
            .pending
            .lock()
            .map_err(|_| ListenError::Other("the callback lock was poisoned".into()))?;

        if pending
            .as_ref()
            .is_some_and(|delivery| !delivery.is_abandoned())
        {
            return Err(ListenError::Other(format!(
                "a callback is already pending for {}",
                self.address
            )));
        }

        *pending = Some(delivery);
        Ok(self.address.clone())
    }
}

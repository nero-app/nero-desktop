#![allow(unused_variables)]

use std::sync::Mutex;

use nero_extensions::{
    CallbackTransport, Cancel, Delivery, ListenError, ProgressHandle, SelectionRequest, Severity,
    TextRequest,
};
use url::Url;

use crate::error::{Error, Result};

use super::ExtensionId;

pub struct Interaction;

#[async_trait::async_trait]
impl nero_extensions::Interaction for Interaction {
    async fn request_text(&self, request: TextRequest) -> Option<String> {
        todo!()
    }

    async fn request_selection(&self, request: SelectionRequest) -> Option<Vec<u32>> {
        todo!()
    }

    fn notify(&self, severity: Severity, message: String, detail: Option<String>) {
        todo!()
    }

    fn begin_progress(
        &self,
        title: String,
        cancellable: bool,
        cancel: Cancel,
    ) -> Box<dyn ProgressHandle> {
        todo!()
    }
}

pub struct Opener;

#[async_trait::async_trait]
impl nero_extensions::Opener for Opener {
    async fn open(&self, uri: &str) -> bool {
        todo!()
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

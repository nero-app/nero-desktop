#![allow(unused_variables)]

use nero_extensions::{
    CallbackTransport, Cancel, Delivery, ListenError, ProgressHandle, SelectionRequest, Severity,
    TextRequest,
};

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

pub struct Callbacks;

impl CallbackTransport for Callbacks {
    fn register(&self, delivery: Delivery) -> Result<String, ListenError> {
        todo!()
    }
}

use std::sync::Arc;

use iced::futures::channel::mpsc;
use iced::futures::{SinkExt, StreamExt};
use iced::widget::stack;
use iced::{Element, Subscription, Task};

pub mod notification;
pub mod progress;
pub mod request;
mod transport;

pub use progress::ProgressId;
pub use request::RequestId;
pub use transport::Transport;

pub enum Event {
    Request(request::Event),
    Notification(notification::Event),
    Progress(progress::Event),
}

pub enum Message {
    Connected(mpsc::UnboundedSender<Event>),
    Received(Event),
    Request(request::Message),
    Notification(notification::Message),
    Progress(progress::Message),
}

pub struct InteractionState {
    transport: Arc<Transport>,
    requests: request::Requests,
    notifications: notification::Notifications,
    progresses: progress::Progresses,
}

impl InteractionState {
    pub fn new() -> (Self, Arc<Transport>) {
        let transport = Arc::new(Transport::default());

        (
            Self {
                transport: transport.clone(),
                requests: request::Requests::default(),
                notifications: notification::Notifications::default(),
                progresses: progress::Progresses::default(),
            },
            transport,
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Connected(sender) => self.transport.connect(sender),
            Message::Received(event) => return self.receive(event),
            Message::Request(message) => {
                return self.requests.update(message).map(Message::Request);
            }
            Message::Notification(message) => {
                self.notifications.update(message);
            }
            Message::Progress(message) => self.progresses.update(message),
        }

        Task::none()
    }

    fn receive(&mut self, event: Event) -> Task<Message> {
        match event {
            Event::Request(event) => {
                return self.requests.receive(event).map(Message::Request);
            }
            Event::Notification(event) => {
                return self.notifications.receive(event).map(Message::Notification);
            }
            Event::Progress(event) => {
                self.progresses.receive(event);
            }
        }

        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let toasts = crate::components::toast::container([
            self.notifications.view().map(Message::Notification),
            self.progresses.view().map(Message::Progress),
        ]);

        stack![toasts, self.requests.view().map(Message::Request)].into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::run(listen)
    }
}

fn listen() -> impl iced::futures::Stream<Item = Message> {
    iced::stream::channel(100, async |mut output| {
        let (sender, mut receiver) = mpsc::unbounded();
        let _ = output.send(Message::Connected(sender)).await;

        while let Some(event) = receiver.next().await {
            if output.send(Message::Received(event)).await.is_err() {
                break;
            }
        }
    })
}

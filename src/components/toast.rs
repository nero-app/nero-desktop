use std::hash::Hash;
use std::time::Duration;

use iced::animation::Easing;
use iced::widget::{column, container as iced_container, float, keyed_column, opaque, transition};
use iced::{Animation, Border, Color, Element, Fill, Rectangle, Vector};

use crate::theme::PALETTE;

const EXIT_DURATION: Duration = Duration::from_millis(240);

const WIDTH: f32 = 360.0;
const FLOATING_EPSILON: f32 = 0.01;

pub struct Toast<'a, Key, Message> {
    key: Key,
    visible: bool,
    border: Border,
    on_finish: Option<Message>,
    content: Box<dyn Fn() -> Element<'a, Message> + 'a>,
}

pub fn toast<'a, Key, Message>(
    key: Key,
    content: impl Fn() -> Element<'a, Message> + 'a,
) -> Toast<'a, Key, Message> {
    Toast {
        key,
        visible: true,
        border: iced::border::rounded(8.0),
        on_finish: None,
        content: Box::new(content),
    }
}

impl<Key, Message> Toast<'_, Key, Message> {
    pub fn border_color(mut self, color: Color) -> Self {
        self.border.color = color;
        self.border.width = 1.0;
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    pub fn on_finish(mut self, message: Message) -> Self {
        self.on_finish = Some(message);
        self
    }
}

impl<'a, Key, Message> Toast<'a, Key, Message>
where
    Key: Copy + Hash + 'a,
    Message: Clone + 'a,
{
    fn into_entry(self) -> (Key, Element<'a, Message>) {
        let Self {
            key,
            visible,
            border,
            on_finish,
            content,
        } = self;
        let element = transition(
            visible,
            || {
                Animation::new(false)
                    .duration(EXIT_DURATION)
                    .easing(Easing::EaseInOut)
            },
            move |animation, now| {
                let visibility = animation.interpolate(0.0, 1.0, now);
                let surface = opaque(iced_container(content()).width(WIDTH).padding(12).style(
                    move |_| iced_container::Style {
                        background: Some(PALETTE.raised.into()),
                        border,
                        ..iced_container::Style::default()
                    },
                ));

                float(surface)
                    .translate(move |bounds, viewport| translation(visibility, bounds, viewport))
            },
        )
        .key(key)
        .on_finish_maybe(on_finish)
        .into();

        (key, element)
    }
}

fn translation(visibility: f32, bounds: Rectangle, viewport: Rectangle) -> Vector {
    let distance = (viewport.x + viewport.width - bounds.x).max(0.0);

    Vector::new((1.0 - visibility) * distance + FLOATING_EPSILON, 0.0)
}

pub fn group<'a, Key, Message>(
    toasts: impl IntoIterator<Item = Toast<'a, Key, Message>>,
) -> Element<'a, Message>
where
    Key: Copy + Hash + PartialEq + 'static,
    Message: Clone + 'a,
{
    keyed_column(toasts.into_iter().map(Toast::into_entry))
        .spacing(8)
        .into()
}

pub fn container<'a, Message>(
    groups: impl IntoIterator<Item = Element<'a, Message>>,
) -> Element<'a, Message>
where
    Message: 'a,
{
    iced_container(column(groups).spacing(8))
        .width(Fill)
        .height(Fill)
        .align_right(Fill)
        .align_bottom(Fill)
        .padding(20)
        .into()
}

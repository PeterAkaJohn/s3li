use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::components::traits::Component;

#[derive(Debug)]
pub struct S3liKeyEvent {
    input: Vec<(KeyCode, KeyModifiers)>,
}
#[derive(Debug)]
pub struct S3liEventWithReaction {}

pub type S3liEventListener<T> = (S3liKeyEvent, fn(&mut T));
pub type S3liWithReactionEventListener<T> =
    (S3liEventWithReaction, fn(&mut T, value: Option<char>));

#[derive(Debug)]
pub enum EventListeners<T> {
    KeyEvent(S3liEventListener<T>),
    EventWithReaction(S3liWithReactionEventListener<T>),
}

impl<T> EventListeners<T> {
    pub fn is_equal(&self, key_event: KeyEvent) -> bool {
        match self {
            EventListeners::KeyEvent(event) => event.0.is_equal(key_event),
            EventListeners::EventWithReaction(event) => event.0.is_equal(key_event),
        }
    }
}

impl S3liKeyEvent {
    pub fn new(input: Vec<(KeyCode, KeyModifiers)>) -> Self {
        Self { input }
    }

    pub fn is_equal(&self, key_event: KeyEvent) -> bool {
        self.input
            .iter()
            .any(|(code, modifiers)| *code == key_event.code && *modifiers == key_event.modifiers)
    }
}
impl S3liEventWithReaction {
    pub fn new() -> Self {
        Self {}
    }

    pub fn is_equal(&self, key_event: KeyEvent) -> bool {
        matches!(key_event.code, KeyCode::Char(_))
    }
}

pub trait ExecuteEventListener
where
    Self: Component + Sized,
{
    fn get_event_listeners(&self) -> &Vec<EventListeners<Self>>;
    fn execute(&mut self, event: KeyEvent) {
        if let Some(actual_event) = self
            .get_event_listeners()
            .iter()
            .find(|eventlistener| eventlistener.is_equal(event))
        {
            match actual_event {
                EventListeners::KeyEvent((_, listener)) => listener(self),
                EventListeners::EventWithReaction((_, listener)) => {
                    if let KeyCode::Char(val) = event.code {
                        listener(self, Some(val))
                    }
                }
            }
        }
    }
}

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::components::traits::Component;

#[derive(Debug)]
pub struct S3liKeyEvent {
    input: Vec<(KeyCode, KeyModifiers)>,
}
#[derive(Debug)]
pub struct S3liOnChangeEvent {}

pub type S3liEventListener<T> = (S3liKeyEvent, fn(&mut T));
pub type S3liOnChangeEventListener<T> = (S3liOnChangeEvent, fn(&mut T, value: char), fn(&mut T));

#[derive(Debug)]
pub enum EventListeners<T> {
    KeyEvent(S3liEventListener<T>),
    OnChangeEvent(S3liOnChangeEventListener<T>),
}

impl<T> EventListeners<T> {
    pub fn is_equal(&self, key_event: KeyEvent) -> bool {
        match self {
            EventListeners::KeyEvent(event) => event.0.is_equal(key_event),
            EventListeners::OnChangeEvent(event) => event.0.is_equal(key_event),
        }
    }
}

impl S3liKeyEvent {
    pub fn new(input: Vec<(KeyCode, KeyModifiers)>) -> Self {
        Self { input }
    }

    pub fn is_equal(&self, key_event: KeyEvent) -> bool {
        self.input.iter().any(|(code, modifiers)| {
            let _key_event_modifiers = &key_event.modifiers;
            *code == key_event.code && matches!(modifiers, _key_event_modifiers)
        })
    }
}
impl S3liOnChangeEvent {
    pub fn new() -> Self {
        Self {}
    }

    pub fn is_equal(&self, key_event: KeyEvent) -> bool {
        matches!(key_event.code, KeyCode::Char(_) | KeyCode::Backspace)
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
                EventListeners::OnChangeEvent((_, add, delete)) => {
                    if let KeyCode::Char(val) = event.code {
                        add(self, val)
                    } else if matches!(event.code, KeyCode::Backspace) {
                        delete(self)
                    }
                }
            }
        }
    }
}

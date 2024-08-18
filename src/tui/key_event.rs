use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::components::traits::Component;

#[derive(Debug, PartialEq)]
pub struct S3liKeyEvent {
    code: KeyCode,
    modifiers: KeyModifiers,
}

pub type S3liEventListener<T> = (S3liKeyEvent, fn(&mut T));

impl S3liKeyEvent {
    pub fn new(code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self { code, modifiers }
    }

    pub fn is_equal_to_crossterm_key_event(&self, key_event: KeyEvent) -> bool {
        self.code == key_event.code && self.modifiers == key_event.modifiers
    }
}

impl From<KeyEvent> for S3liKeyEvent {
    fn from(value: KeyEvent) -> Self {
        Self {
            code: value.code,
            modifiers: value.modifiers,
        }
    }
}

pub trait ExecuteEventListener
where
    Self: Component + Sized,
{
    fn get_event_listeners(&self) -> &Vec<S3liEventListener<Self>>;
    fn execute(&mut self, event: KeyEvent) {
        if let Some((_, listener)) = self.get_event_listeners().iter().find(|(key, _)| {
            let s3lievent: S3liKeyEvent = event.into();
            s3lievent == *key
        }) {
            listener(self)
        }
    }
}

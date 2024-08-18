use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::components::traits::Component;

#[derive(Debug)]
pub struct S3liKeyEvent {
    input: Vec<(KeyCode, KeyModifiers)>,
}

pub type S3liEventListener<T> = (S3liKeyEvent, fn(&mut T));

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

pub trait ExecuteEventListener
where
    Self: Component + Sized,
{
    fn get_event_listeners(&self) -> &Vec<S3liEventListener<Self>>;
    fn execute(&mut self, event: KeyEvent) {
        if let Some((_, listener)) = self
            .get_event_listeners()
            .iter()
            .find(|(key, _)| key.is_equal(event))
        {
            listener(self)
        }
    }
}

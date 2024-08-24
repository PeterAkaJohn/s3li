use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::components::traits::Component;

#[derive(Debug)]
pub struct S3liKeyEvent {
    input: Vec<(KeyCode, KeyModifiers)>,
    description: String,
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
    pub fn new(input: Vec<(KeyCode, KeyModifiers)>, description: String) -> Self {
        Self { input, description }
    }

    pub fn is_equal(&self, key_event: KeyEvent) -> bool {
        self.input.iter().any(|(code, modifiers)| {
            let _key_event_modifiers = &key_event.modifiers;
            *code == key_event.code && matches!(modifiers, _key_event_modifiers)
        })
    }

    pub fn get_description(&self) -> &str {
        self.description.as_str()
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

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyEventKind, KeyEventState, KeyModifiers};

    use crate::tui::{components::traits::Component, key_event::S3liOnChangeEvent};

    use super::{EventListeners, ExecuteEventListener, S3liKeyEvent};

    struct MockComponent {
        listeners: Vec<EventListeners<Self>>,
        state: String,
    }

    impl MockComponent {
        fn update_state(&mut self) {
            self.state = "update_from_test".to_string()
        }

        fn add_char(&mut self, value: char) {
            self.state.push(value);
        }
        fn delete_char(&mut self) {
            self.state.pop();
        }
    }

    impl Component for MockComponent {
        fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) {
            self.execute(key);
        }

        fn render(
            &mut self,
            _f: &mut ratatui::Frame,
            _area: ratatui::prelude::Rect,
            _props: Option<crate::tui::components::traits::ComponentProps>,
        ) {
        }
    }

    impl ExecuteEventListener for MockComponent {
        fn get_event_listeners(&self) -> &Vec<EventListeners<Self>> {
            &self.listeners
        }
    }

    #[test]
    fn test_execute_event_listeners_key_events() {
        let mut mock_component = MockComponent {
            listeners: vec![EventListeners::KeyEvent((
                S3liKeyEvent::new(
                    vec![(crossterm::event::KeyCode::Char('t'), KeyModifiers::NONE)],
                    "".into(),
                ),
                MockComponent::update_state,
            ))],
            state: "test".into(),
        };

        mock_component.handle_key_events(crossterm::event::KeyEvent {
            code: crossterm::event::KeyCode::Backspace,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        });

        assert_eq!(mock_component.state, "test".to_string());

        mock_component.handle_key_events(crossterm::event::KeyEvent {
            code: crossterm::event::KeyCode::Char('t'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        });

        assert_eq!(mock_component.state, "update_from_test".to_string());
    }

    #[test]
    fn test_execute_event_listeners_onchange_event() {
        let mut mock_component = MockComponent {
            listeners: vec![EventListeners::OnChangeEvent((
                S3liOnChangeEvent::new(),
                MockComponent::add_char,
                MockComponent::delete_char,
            ))],
            state: "test".into(),
        };

        mock_component.handle_key_events(crossterm::event::KeyEvent {
            code: crossterm::event::KeyCode::Tab,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        });

        assert_eq!(mock_component.state, "test".to_string());

        mock_component.handle_key_events(crossterm::event::KeyEvent {
            code: crossterm::event::KeyCode::Char('x'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        });

        assert_eq!(mock_component.state, "testx".to_string());

        mock_component.handle_key_events(crossterm::event::KeyEvent {
            code: crossterm::event::KeyCode::Backspace,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        });

        assert_eq!(mock_component.state, "test".to_string());
    }
}

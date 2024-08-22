use crossterm::event::KeyModifiers;
use ratatui::{
    layout::{Constraint, Layout},
    widgets::Clear,
};

use crate::tui::{
    components::{
        input::InputBlock,
        popup::WithPopup,
        traits::{Component, ComponentProps, WithContainer},
    },
    key_event::{EventListeners, ExecuteEventListener, S3liKeyEvent, S3liOnChangeEvent},
};

enum Selected {
    Name,
    Value,
}

pub struct AddProperty {
    name: String,
    value: String,
    open: bool,
    selected: Selected,
    listeners: Vec<EventListeners<Self>>,
}

impl AddProperty {
    pub fn new() -> Self {
        Self {
            name: Default::default(),
            value: Default::default(),
            open: false,
            selected: Selected::Name,
            listeners: Self::register_listeners(),
        }
    }

    pub fn toggle_selected(&mut self) {
        self.selected = match self.selected {
            Selected::Name => Selected::Value,
            Selected::Value => Selected::Name,
        };
    }

    pub fn get_property_to_add(&mut self) -> (String, Option<String>) {
        let new_property = (self.name.clone(), Some(self.value.clone()));
        self.name = Default::default();
        self.value = Default::default();
        new_property
    }

    fn register_listeners() -> Vec<EventListeners<Self>> {
        vec![
            EventListeners::KeyEvent((
                S3liKeyEvent::new(vec![(crossterm::event::KeyCode::Esc, KeyModifiers::NONE)]),
                Self::exit_component,
            )),
            EventListeners::KeyEvent((
                S3liKeyEvent::new(vec![(crossterm::event::KeyCode::Tab, KeyModifiers::NONE)]),
                Self::toggle_selected,
            )),
            EventListeners::KeyEvent((
                S3liKeyEvent::new(vec![(
                    crossterm::event::KeyCode::Backspace,
                    KeyModifiers::NONE,
                )]),
                Self::delete_char_from_selected,
            )),
            EventListeners::OnChangeEvent((
                S3liOnChangeEvent::new(),
                Self::add_char_to_selected,
                Self::delete_char_from_selected,
            )),
        ]
    }

    fn exit_component(&mut self) {
        self.close_popup();
    }

    fn delete_char_from_selected(&mut self) {
        match self.selected {
            Selected::Name => self.name.pop(),
            Selected::Value => self.value.pop(),
        };
    }
    fn add_char_to_selected(&mut self, value: char) {
        match self.selected {
            Selected::Name => self.name.push(value),
            Selected::Value => self.value.push(value),
        };
    }
}

impl WithPopup for AddProperty {
    fn set_popup_state(&mut self, open: bool) {
        self.open = open;
    }

    fn get_popup_state(&self) -> bool {
        self.open
    }
}

impl WithContainer<'_> for AddProperty {}

impl ExecuteEventListener for AddProperty {
    fn get_event_listeners(&self) -> &Vec<EventListeners<Self>> {
        &self.listeners
    }
}

impl Component for AddProperty {
    fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) {
        self.execute(key);
    }

    fn render(
        &mut self,
        f: &mut ratatui::Frame,
        _area: ratatui::prelude::Rect,
        _props: Option<ComponentProps>,
    ) {
        let container =
            self.with_container("Add Property", &Some(ComponentProps { selected: true }));

        let horizontal = Layout::horizontal([Constraint::Fill(1); 3]).split(f.size());
        let vertical = Layout::vertical([Constraint::Fill(1); 3]).split(horizontal[1]);

        f.render_widget(Clear, vertical[1]);
        f.render_widget(container, vertical[1]);

        let input_container = Layout::vertical([Constraint::Length(3), Constraint::Fill(1)])
            .margin(1)
            .split(vertical[1]);

        let input_name = InputBlock::new(
            self.name.clone(),
            "Name".to_string(),
            matches!(self.selected, Selected::Name),
        );

        f.render_widget(input_name, input_container[0]);

        let input_value = InputBlock::new(
            self.value.clone(),
            "Value".to_string(),
            matches!(self.selected, Selected::Value),
        );

        f.render_widget(input_value, input_container[1]);
    }
}

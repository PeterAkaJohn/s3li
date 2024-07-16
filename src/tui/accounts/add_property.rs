use ratatui::{
    layout::{Constraint, Layout},
    widgets::Clear,
};

use crate::tui::{
    components::{
        input::Input,
        traits::{Component, ComponentProps, WithContainer},
    },
    popup::WithPopup,
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
}

impl AddProperty {
    pub fn new() -> Self {
        Self {
            name: Default::default(),
            value: Default::default(),
            open: false,
            selected: Selected::Name,
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

impl Component for AddProperty {
    fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) {
        //esc closes, tab switches between name and value, rest of value are for writing value
        match key.code {
            crossterm::event::KeyCode::Esc => {
                self.close_popup();
            }
            crossterm::event::KeyCode::Tab => {
                self.toggle_selected();
            }
            crossterm::event::KeyCode::Backspace => {
                match self.selected {
                    Selected::Name => self.name.pop(),
                    Selected::Value => self.value.pop(),
                };
            }
            crossterm::event::KeyCode::Char(character) => {
                match self.selected {
                    Selected::Name => self.name.push(character),
                    Selected::Value => self.value.push(character),
                };
            }
            crossterm::event::KeyCode::Enter => {
                //something that I need to figure out
            }
            _ => {}
        }
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

        let input_name = Input::new(
            self.name.clone(),
            "Name".to_string(),
            matches!(self.selected, Selected::Name),
        );

        f.render_widget(input_name, input_container[0]);

        let input_value = Input::new(
            self.value.clone(),
            "Value".to_string(),
            matches!(self.selected, Selected::Value),
        );

        f.render_widget(input_value, input_container[1]);
    }
}

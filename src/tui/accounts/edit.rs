use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::Clear,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action,
    logger::LOGGER,
    tui::{
        component::{Component, ComponentProps, WithContainer},
        components::input::Input,
        popup::WithPopup,
    },
};

use super::add_property::AddProperty;

pub struct EditAccount {
    open: bool,
    properties: HashMap<String, Option<String>>,
    new_properties: Vec<(String, Option<String>)>,
    account_to_edit: Option<String>,
    selected_idx: usize,
    ui_tx: UnboundedSender<Action>,
    add_property: AddProperty,
}

impl EditAccount {
    pub fn new(ui_tx: UnboundedSender<Action>) -> Self {
        Self {
            open: false,
            account_to_edit: None,
            properties: HashMap::new(),
            new_properties: vec![],
            ui_tx: ui_tx.clone(),
            selected_idx: 0,
            add_property: AddProperty::new(ui_tx.clone()),
        }
    }

    pub fn update_properties(
        &mut self,
        account: String,
        properties: HashMap<String, Option<String>>,
    ) {
        self.account_to_edit = Some(account);
        self.properties = properties.clone();
        self.new_properties = properties
            .clone()
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_owned()))
            .collect::<Vec<(String, Option<String>)>>();
        self.selected_idx = 0;
    }
}

impl WithContainer<'_> for EditAccount {}

impl WithPopup for EditAccount {
    fn set_popup_state(&mut self, open: bool) {
        self.open = open;
    }

    fn get_popup_state(&self) -> bool {
        self.open
    }
}

impl Component for EditAccount {
    fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) {
        if self.add_property.is_popup_open() {
            self.add_property.handle_key_events(key);
            return;
        }
        if let KeyEvent {
            code: KeyCode::Char('a'),
            modifiers: KeyModifiers::CONTROL | KeyModifiers::SHIFT,
            ..
        } = key
        {
            self.add_property.open_popup();
            return;
        };
        match key.code {
            crossterm::event::KeyCode::Esc => {
                self.close_popup();
            }
            crossterm::event::KeyCode::Tab => {
                if self.selected_idx == self.new_properties.len() - 1 {
                    self.selected_idx = 0;
                } else {
                    self.selected_idx += 1;
                }
            }
            crossterm::event::KeyCode::Backspace => {
                if let Some(item) = self.new_properties.get_mut(self.selected_idx) {
                    if let Some(act_val) = item.1.as_mut() {
                        act_val.pop();
                    }
                }
            }
            crossterm::event::KeyCode::Char(character) => {
                if let Some(item) = self.new_properties.get_mut(self.selected_idx) {
                    if let Some(value) = item.1.as_mut() {
                        value.push(character)
                    }
                }
            }
            crossterm::event::KeyCode::Enter => {
                if !self.new_properties.is_empty() {
                    let new_properties_hash_map = self
                        .new_properties
                        .iter()
                        .map(|(key, value)| (key.to_owned(), value.to_owned()))
                        .collect::<HashMap<String, Option<String>>>();
                    let _ = self.ui_tx.send(Action::EditCredentials(
                        self.account_to_edit.clone().unwrap(),
                        new_properties_hash_map,
                    ));
                }
            }
            _ => {
                LOGGER.info(&format!("{:#?}", key.code));
            }
        }
    }

    fn render(
        &mut self,
        f: &mut ratatui::prelude::Frame,
        _area: ratatui::prelude::Rect,
        _props: Option<ComponentProps>,
    ) {
        let name = &self.account_to_edit;
        if name.is_none() {
            panic!("name cannot be None when editing");
        }
        if let Some(name) = name {
            let title = &format!("Edit Account {name}");
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(15), Constraint::Percentage(85)])
                .split(f.size());
            let section = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Fill(1), Constraint::Length(3)])
                .split(layout[1]);
            let container = self.with_container(title, &Some(ComponentProps { selected: true }));
            f.render_widget(Clear, section[0]);
            f.render_widget(container, section[0]);

            let inner_layout = Layout::default()
                .margin(2)
                .direction(Direction::Vertical)
                .constraints(
                    self.new_properties
                        .iter()
                        .enumerate()
                        .map(|(idx, _)| {
                            if self.selected_idx == idx {
                                Constraint::Fill(1)
                            } else {
                                Constraint::Max(3)
                            }
                        })
                        .collect::<Vec<Constraint>>(),
                )
                .split(section[0]);

            self.new_properties
                .iter()
                .zip(inner_layout.iter())
                .enumerate()
                .for_each(|(idx, ((key, value), property_area))| {
                    if let Some(value) = value {
                        let is_selected = idx == self.selected_idx;
                        let input = Input::new(value.to_string(), key.to_string(), is_selected);
                        f.render_widget(input, *property_area);
                    }
                });
            if self.add_property.is_popup_open() {
                self.add_property.render(f, _area, _props);
            }
        }
    }
}

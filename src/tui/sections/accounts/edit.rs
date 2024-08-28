use std::{collections::HashMap, ops::Not};

use anyhow::{Ok, Result};
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::Clear,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action,
    tui::{
        components::{
            input::InputBlock,
            popup::WithPopup,
            traits::{Component, ComponentProps, WithContainer},
        },
        key_event::{EventListeners, ExecuteEventListener, S3liKeyEvent, S3liOnChangeEvent},
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
    listeners: Vec<EventListeners<Self>>,
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
            add_property: AddProperty::new(),
            listeners: Self::register_listeners(),
        }
    }

    fn add_to_properties(&mut self, property: (String, Option<String>)) -> Result<bool> {
        self.new_properties.push(property);
        Ok(true)
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

    fn register_listeners() -> Vec<EventListeners<Self>> {
        vec![
            EventListeners::KeyEvent((
                S3liKeyEvent::new(
                    vec![(crossterm::event::KeyCode::Char('a'), KeyModifiers::CONTROL)],
                    "Add property: <C>-a".into(),
                ),
                Self::open_add_property,
            )),
            EventListeners::KeyEvent((
                S3liKeyEvent::new(
                    vec![(crossterm::event::KeyCode::Esc, KeyModifiers::NONE)],
                    "Cancel: <Esc>".into(),
                ),
                Self::exit_component,
            )),
            EventListeners::KeyEvent((
                S3liKeyEvent::new(
                    vec![(crossterm::event::KeyCode::Enter, KeyModifiers::NONE)],
                    "Confirm: <Enter>".into(),
                ),
                Self::confirm,
            )),
            EventListeners::KeyEvent((
                S3liKeyEvent::new(
                    vec![(crossterm::event::KeyCode::Tab, KeyModifiers::NONE)],
                    "Cycle: <Tab>".into(),
                ),
                Self::cycle_properties,
            )),
            EventListeners::OnChangeEvent((
                S3liOnChangeEvent::new(),
                Self::add_char,
                Self::delete_char,
            )),
        ]
    }

    fn open_add_property(&mut self) {
        if !self.add_property.is_popup_open() {
            self.add_property.open_popup();
        }
    }
    fn exit_component(&mut self) {
        self.close_popup();
    }
    fn confirm(&mut self) {
        if self.add_property.is_popup_open() {
            let new_property = self.add_property.get_property_to_add();
            let _ = self.add_to_properties(new_property);
            self.add_property.close_popup();
        } else if !self.new_properties.is_empty() {
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
    fn cycle_properties(&mut self) {
        if self.selected_idx == self.new_properties.len() - 1 {
            self.selected_idx = 0;
        } else {
            self.selected_idx += 1;
        }
    }
    fn delete_char(&mut self) {
        if let Some(item) = self.new_properties.get_mut(self.selected_idx) {
            if let Some(act_val) = item.1.as_mut() {
                act_val.pop();
            }
        }
    }
    fn add_char(&mut self, value: char) {
        if let Some(item) = self.new_properties.get_mut(self.selected_idx) {
            if let Some(prop) = item.1.as_mut() {
                prop.push(value)
            }
        }
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

impl ExecuteEventListener for EditAccount {
    fn get_event_listeners(&self) -> &Vec<EventListeners<Self>> {
        &self.listeners
    }
}

impl Component for EditAccount {
    fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) {
        if self.add_property.is_popup_open() && matches!(key.code, KeyCode::Enter).not() {
            self.add_property.handle_key_events(key);
            return;
        }
        self.execute(key);
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
                .constraints([Constraint::Fill(1), Constraint::Length(4)])
                .split(layout[1]);
            let container = self.with_container(title, &Some(ComponentProps { selected: true }));
            f.render_widget(Clear, section[0]);
            f.render_widget(container, section[0]);

            let inner_layout = Layout::default()
                .direction(Direction::Vertical)
                .horizontal_margin(2)
                .vertical_margin(1)
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
                        let input =
                            InputBlock::new(value.to_string(), key.to_string(), is_selected)
                                .with_title_alignment(ratatui::layout::Alignment::Right);
                        f.render_widget(input, *property_area);
                    }
                });
            if self.add_property.is_popup_open() {
                self.add_property.render(f, _area, _props);
            }
        }
    }
}

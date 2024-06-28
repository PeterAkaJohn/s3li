use std::collections::HashMap;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action,
    tui::{
        component::{Component, ComponentProps, WithContainer},
        popup::WithPopup,
    },
};

pub struct EditAccount {
    open: bool,
    properties: HashMap<String, Option<String>>,
    new_properties: HashMap<String, Option<String>>,
    account_to_edit: Option<String>,
    ui_tx: UnboundedSender<Action>,
}

impl EditAccount {
    pub fn new(ui_tx: UnboundedSender<Action>) -> Self {
        Self {
            open: false,
            account_to_edit: None,
            properties: HashMap::new(),
            new_properties: HashMap::new(),
            ui_tx,
        }
    }

    pub fn update_properties(
        &mut self,
        account: String,
        properties: HashMap<String, Option<String>>,
    ) {
        self.account_to_edit = Some(account);
        self.properties = properties.clone();
        self.new_properties = properties.clone();
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
        match key.code {
            crossterm::event::KeyCode::Esc => {
                self.close_popup();
            }
            _ => {}
        }
    }

    fn render(
        &mut self,
        f: &mut ratatui::prelude::Frame,
        _area: ratatui::prelude::Rect,
        props: Option<ComponentProps>,
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
            let container = self.with_container(title, &Some(ComponentProps { selected: true }));
            f.render_widget(Clear, layout[1]);
            f.render_widget(container, layout[1]);

            let inner_layout = Layout::default()
                .margin(2)
                .direction(Direction::Vertical)
                .constraints(
                    self.new_properties
                        .keys()
                        .map(|_| Constraint::Max(3))
                        .collect::<Vec<Constraint>>(),
                )
                .split(layout[1]);

            self.new_properties
                .iter()
                .zip(inner_layout.iter())
                .for_each(|((key, value), property_area)| {
                    if let Some(value) = value {
                        let input_sections = Layout::default()
                            .direction(Direction::Horizontal)
                            .constraints([Constraint::Fill(1)])
                            .split(*property_area);
                        let input_value = Paragraph::new(value.to_string())
                            .wrap(Wrap::default())
                            .block(
                                Block::new()
                                    .title(key.to_string())
                                    .title_alignment(Alignment::Center)
                                    .borders(Borders::ALL)
                                    .border_type(BorderType::Rounded),
                            );
                        f.render_widget(Clear, input_sections[0]);
                        f.render_widget(input_value, input_sections[0]);
                    }
                });
        }
    }
}

use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Clear, Paragraph},
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action,
    tui::{
        components::traits::{Component, ComponentProps, WithContainer},
        popup::WithPopup,
    },
};

#[derive(Debug)]
pub struct Download {
    pub open: bool,
    pub file_name: Option<String>,
    pub key: Option<String>,
    ui_tx: UnboundedSender<Action>,
}

impl Download {
    pub fn new(ui_tx: UnboundedSender<Action>) -> Download {
        Download {
            ui_tx,
            open: false,
            file_name: None,
            key: None,
        }
    }

    pub fn init(&mut self, key: String) {
        let file_name = key
            .split('/')
            .collect::<Vec<&str>>()
            .pop()
            .map(|val| val.to_string());
        self.file_name = file_name;
        self.key = Some(key);
        self.open = true;
    }
}

impl WithPopup for Download {
    fn set_popup_state(&mut self, open: bool) {
        self.open = open;
    }

    fn get_popup_state(&self) -> bool {
        self.open
    }
}

impl WithContainer<'_> for Download {}

impl Component for Download {
    fn render(
        &mut self,
        f: &mut ratatui::prelude::Frame,
        _area: ratatui::prelude::Rect,
        props: Option<ComponentProps>,
    ) {
        if let Some(file_name) = &self.file_name {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Fill(1), Constraint::Max(3), Constraint::Fill(1)])
                .split(f.size());
            let center_section = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Fill(2),
                    Constraint::Fill(1),
                    Constraint::Fill(2),
                ])
                .split(layout[1])[1];

            let container = self.with_container("Download file", &props);
            let input_value = Paragraph::new(file_name.to_string()).block(container);
            f.render_widget(Clear, center_section);
            f.render_widget(input_value, center_section);
        }
    }
    fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) {
        match key.code {
            crossterm::event::KeyCode::Esc => {
                self.open = false;
            }
            crossterm::event::KeyCode::Enter => {
                // send region to state with ui_tx
                match (self.key.clone(), self.file_name.clone()) {
                    (Some(file_key), Some(file_name)) => {
                        let _ = self.ui_tx.send(Action::DownloadFile(file_key, file_name));
                    }
                    _ => panic!("cannot happen must have key and filename"),
                }
                self.open = false;
            }
            crossterm::event::KeyCode::Backspace => {
                // send region to state with ui_tx
                if let Some(file_name) = self.file_name.as_mut() {
                    file_name.pop();
                };
            }
            crossterm::event::KeyCode::Char(value) => {
                if let Some(file_name) = self.file_name.as_mut() {
                    file_name.push(value)
                }
            }
            _ => {}
        }
    }
}

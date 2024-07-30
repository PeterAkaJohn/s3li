use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Clear, Paragraph},
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action,
    tui::components::{
        popup::WithPopup,
        traits::{Component, ComponentProps, WithContainer},
    },
};

#[derive(Debug)]
pub struct FileToDownload {
    pub file_name: String,
    pub key: String,
}

impl From<String> for FileToDownload {
    fn from(key: String) -> Self {
        let file_name = key
            .split('/')
            .collect::<Vec<&str>>()
            .pop()
            .map(|val| val.to_string());
        Self {
            file_name: file_name.expect("file name in download should never be None"),
            key,
        }
    }
}

#[derive(Debug)]
pub struct Download {
    pub open: bool,
    pub files: Vec<FileToDownload>,
    ui_tx: UnboundedSender<Action>,
}

impl Download {
    pub fn new(ui_tx: UnboundedSender<Action>) -> Download {
        Download {
            ui_tx,
            open: false,
            files: vec![],
        }
    }

    pub fn init(&mut self, keys: Vec<String>) {
        self.files = keys.iter().map(|key| (key.clone()).into()).collect();
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
        let file = self.files.get(0);
        if let Some(file) = &file {
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
            let input_value = Paragraph::new(file.file_name.to_string()).block(container);
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
                let file = self.files.first();
                if let Some(file) = file {
                    let _ = self.ui_tx.send(Action::DownloadFile(
                        file.key.clone(),
                        file.file_name.clone(),
                    ));
                }
                self.open = false;
            }
            crossterm::event::KeyCode::Backspace => {
                // send region to state with ui_tx
                if let Some(file) = self.files.get_mut(0) {
                    file.file_name.pop();
                }
            }
            crossterm::event::KeyCode::Char(value) => {
                if let Some(file) = self.files.get_mut(0) {
                    file.file_name.push(value);
                }
            }
            _ => {}
        }
    }
}

use std::ops::Add;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{block::Title, Clear, Paragraph},
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action,
    store::explorer::FileToDownload,
    tui::components::{
        popup::WithPopup,
        traits::{Component, ComponentProps, WithContainer},
    },
};

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
    current_file_idx: usize,
}

impl Download {
    pub fn new(ui_tx: UnboundedSender<Action>) -> Download {
        Download {
            ui_tx,
            open: false,
            files: vec![],
            current_file_idx: 0,
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
        let mut container = self.with_container("Download file", &props);
        container = container.title(
            Title::from(format!(
                "{} of {}",
                self.current_file_idx + 1,
                self.files.len()
            ))
            .position(ratatui::widgets::block::Position::Bottom)
            .alignment(ratatui::layout::Alignment::Right),
        );

        let current_file = self.files.get(self.current_file_idx);
        if let Some(file) = &current_file {
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
                let _ = self.ui_tx.send(Action::DownloadFile(self.files.clone()));
                self.open = false;
            }
            crossterm::event::KeyCode::Tab => {
                self.current_file_idx = if self.current_file_idx == self.files.len() - 1 {
                    0
                } else {
                    self.current_file_idx.add(1)
                };
            }
            crossterm::event::KeyCode::Backspace => {
                // send region to state with ui_tx
                if let Some(file) = self.files.get_mut(self.current_file_idx) {
                    file.file_name.pop();
                }
            }
            crossterm::event::KeyCode::Char(value) => {
                if let Some(file) = self.files.get_mut(self.current_file_idx) {
                    file.file_name.push(value);
                }
            }
            _ => {}
        }
    }
}

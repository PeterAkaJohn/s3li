use std::ops::Add;

use ratatui::{
    layout::{Constraint, Direction, Layout, Margin},
    widgets::{block::Title, Clear},
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action,
    store::explorer::TreeItem,
    tui::components::{
        input::Input,
        popup::WithPopup,
        traits::{Component, ComponentProps, WithContainer},
    },
};

#[derive(Debug)]
pub struct Download {
    pub open: bool,
    pub items: Vec<TreeItem>,
    ui_tx: UnboundedSender<Action>,
    current_file_idx: usize,
}

impl Download {
    pub fn new(ui_tx: UnboundedSender<Action>) -> Download {
        Download {
            ui_tx,
            open: false,
            items: vec![],
            current_file_idx: 0,
        }
    }

    pub fn init(&mut self, tree_items: Vec<TreeItem>) {
        self.items = tree_items;
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
                self.items.len()
            ))
            .position(ratatui::widgets::block::Position::Bottom)
            .alignment(ratatui::layout::Alignment::Right),
        );

        let current_item = self.items.get(self.current_file_idx);
        if let Some(item) = current_item {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Fill(1), Constraint::Max(3), Constraint::Fill(1)])
                .split(f.size());
            let center_section = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Fill(2),
                    Constraint::Fill(2),
                    Constraint::Fill(2),
                ])
                .split(layout[1])[1];

            let input = Input::new(item.name().to_string(), true);
            f.render_widget(Clear, center_section);
            f.render_widget(container, center_section);
            f.render_widget(input, center_section.inner(&Margin::new(1, 1)));
        }
    }
    fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) {
        match key.code {
            crossterm::event::KeyCode::Esc => {
                self.open = false;
            }
            crossterm::event::KeyCode::Enter => {
                // send region to state with ui_tx
                let _ = self.ui_tx.send(Action::Download(self.items.clone()));
                self.open = false;
            }
            crossterm::event::KeyCode::Tab => {
                self.current_file_idx = if self.current_file_idx == self.items.len() - 1 {
                    0
                } else {
                    self.current_file_idx.add(1)
                };
            }
            crossterm::event::KeyCode::Backspace => {
                // send region to state with ui_tx
                if let Some(item) = self.items.get_mut(self.current_file_idx) {
                    item.pop_name_char();
                }
            }
            crossterm::event::KeyCode::Char(value) => {
                if let Some(item) = self.items.get_mut(self.current_file_idx) {
                    item.push_name_char(value);
                }
            }
            _ => {}
        }
    }
}

use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Clear, Paragraph},
};
use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;

use super::{
    component::{Component, ComponentProps, WithContainer},
    popup::WithPopup,
};

pub struct Region {
    pub open: bool,
    pub region: String,
    pub new_region: String,
    ui_tx: UnboundedSender<Action>,
}

impl Region {
    pub fn new(region: String, ui_tx: UnboundedSender<Action>) -> Region {
        Region {
            ui_tx,
            open: false,
            region: region.clone(),
            new_region: region.clone(),
        }
    }
}

impl WithPopup for Region {
    fn set_popup_state(&mut self, open: bool) {
        self.open = open;
    }

    fn get_popup_state(&self) -> bool {
        self.open
    }
}

impl WithContainer<'_> for Region {}

impl Component for Region {
    fn render(
        &mut self,
        f: &mut ratatui::prelude::Frame,
        _area: ratatui::prelude::Rect,
        props: Option<ComponentProps>,
    ) {
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

        let container = self.with_container("Region", &props);
        let input_value = Paragraph::new(self.new_region.clone()).block(container);
        f.render_widget(Clear, center_section);
        f.render_widget(input_value, center_section);
    }
    fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) {
        if !self.is_popup_open() && key.code == crossterm::event::KeyCode::Char('r') {
            return self.open_popup();
        }
        match key.code {
            crossterm::event::KeyCode::Esc => {
                self.open = false;
                if !self.new_region.eq(&self.region) {
                    self.new_region.clone_from(&self.region);
                }
            }
            crossterm::event::KeyCode::Enter => {
                // send region to state with ui_tx
                let _ = self
                    .ui_tx
                    .send(Action::ChangeRegion(self.new_region.clone()));
                self.open = false;
            }
            crossterm::event::KeyCode::Backspace => {
                // send region to state with ui_tx
                self.new_region.pop();
            }
            crossterm::event::KeyCode::Char(value) => {
                self.new_region.push(value);
            }
            _ => {}
        }
    }
}

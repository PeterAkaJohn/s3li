use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Paragraph},
};
use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;

use super::{
    component::{Component, ComponentProps, WithContainer},
    list::ListComponent,
};

pub struct Region {
    pub open: bool,
    ui_tx: UnboundedSender<Action>,
}

impl Region {
    pub fn new(ui_tx: UnboundedSender<Action>) -> Region {
        Region { ui_tx, open: false }
    }
}

impl WithContainer<'_> for Region {}

impl Component for Region {
    fn render(
        &mut self,
        f: &mut ratatui::prelude::Frame,
        area: ratatui::prelude::Rect,
        props: Option<ComponentProps>,
    ) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(75 / 2),
                Constraint::Percentage(25),
                Constraint::Percentage(75 / 2),
            ])
            .split(f.size());
        let center_section = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ])
            .split(layout[1])[1];

        let container = self.with_container("Region", &props);

        let paragraph = Paragraph::new("region section").block(container);
        f.render_widget(paragraph, center_section);
    }
    fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) {
        if let crossterm::event::KeyCode::Esc = key.code {
            self.open = false
        };
    }
}

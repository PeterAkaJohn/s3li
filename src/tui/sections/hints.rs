use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::tui::components::traits::{Component, ComponentProps};

pub struct Hints {
    hints: Vec<String>,
}

impl Hints {
    pub fn new(hints: Vec<String>) -> Hints {
        Self { hints }
    }

    fn listeners_to_string(&self) -> String {
        self.hints.join(" | ")
    }
}

impl Component for Hints {
    fn render(
        &mut self,
        f: &mut ratatui::prelude::Frame,
        area: ratatui::prelude::Rect,
        _props: Option<ComponentProps>,
    ) {
        let container = Block::default().borders(Borders::NONE);
        if !self.hints.is_empty() {
            let inner_container = container.inner(area);
            let style = Style::default().fg(Color::LightBlue);
            let text = Paragraph::new(self.listeners_to_string()).style(style);
            f.render_widget(text, inner_container);
        }
        f.render_widget(container, area);
    }
    fn handle_key_events(&mut self, _key: crossterm::event::KeyEvent) {}
}

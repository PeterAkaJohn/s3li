use crossterm::event::KeyEventKind;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{block::Title, Block, Borders, List, ListItem, ListState, Padding},
};
use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;

use super::{
    component::{Component, ComponentProps},
    list::WithContainer,
};

#[derive(Debug)]
pub struct Explorer<'a> {
    selected_file: Option<&'a str>,
    ui_tx: UnboundedSender<Action>,
}

impl<'a> Explorer<'a> {
    pub fn new(ui_tx: UnboundedSender<Action>) -> Self {
        Self {
            selected_file: None,
            ui_tx,
        }
    }
}

impl WithContainer<'_> for Explorer<'_> {}

impl Component for Explorer<'_> {
    fn render(
        &mut self,
        f: &mut ratatui::prelude::Frame,
        area: ratatui::prelude::Rect,
        props: Option<ComponentProps>,
    ) {
        let explorer = self.with_container("Explorer", props);

        f.render_widget(explorer, area);
    }

    fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            _ => {}
        };
    }
}

use crossterm::event::{KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{block::Title, Block, Borders, List, ListItem, Padding},
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{action::Action, state::state::AppState};

use super::{component::Component, sources::Sources};

pub struct Dashboard {
    selected_component: DashboardComponents,
    sources: Box<dyn Component>,
    ui_tx: UnboundedSender<Action>,
}

enum DashboardComponents {
    Sources,
    Accounts,
    Explorer
}

impl Dashboard {
    pub fn new(state: &AppState, ui_tx: UnboundedSender<Action>) -> Self {
        let sources =Box::new(Sources::new(
            vec![
                "test1", "test2", "test3",
            "test1", "test2", "test3", "test1", "test2", "test3", "test1", "test2", "test3",
"test1", "test2", "test3", "test1", "test2", "test3", "test1", "test2", "test3", "test1", "test2", "test3"
            ],
            ui_tx.clone(),
        )); 
        Self {
            selected_component: DashboardComponents::Sources,
            sources,
            ui_tx
        }
    }
}

impl Component for Dashboard {
    fn render(&mut self, f: &mut ratatui::prelude::Frame, area: Rect, props: ()) {
        let [aside, main] = *Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(f.size())
        else {
            panic!("layout needs to have 2 chunks")
        };
        self.sources.render(f, aside, props)
    }

    fn handle_key_events(&mut self, key: KeyEvent) {
        match self.selected_component {
            DashboardComponents::Sources => self.sources.handle_key_events(key),
            DashboardComponents::Accounts => todo!(),
            DashboardComponents::Explorer => todo!(),
        }
    }
}

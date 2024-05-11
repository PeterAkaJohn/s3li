use core::panic;

use crossterm::event::{KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{block::Title, Block, Borders, List, ListItem, Padding},
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{action::Action, state::state::AppState};

use super::{
    // accounts::Accounts,
    accounts::Accounts,
    component::{Component, ComponentProps},
    explorer::Explorer,
    list::ListComponent,
    sources::Sources, // sources::Sources,
};

pub struct Dashboard<'a> {
    selected_component: DashboardComponents,
    sources: Sources<'a>,
    accounts: Accounts<'a>,
    explorer: Explorer<'a>,
    ui_tx: UnboundedSender<Action>,
}

enum DashboardComponents {
    Sources,
    Accounts,
    Explorer,
}

impl Dashboard<'_> {
    pub fn new(state: &AppState, ui_tx: UnboundedSender<Action>) -> Self {
        let sources = Sources::new(
            vec![
                "test1", "test2", "test3", "test1", "test2", "test3", "test1", "test2", "test3",
                "test1", "test2", "test3", "test1", "test2", "test3", "test1", "test2", "test3",
                "test1", "test2", "test3", "test1", "test2", "test3",
            ],
            ui_tx.clone(),
        );
        let accounts = Accounts::new(
            vec![
                "test1", "test2", "test3", "test1", "test2", "test3", "test1", "test2", "test3",
                "test1", "test2", "test3", "test1", "test2", "test3", "test1", "test2", "test3",
                "test1", "test2", "test3", "test1", "test2", "test3",
            ],
            ui_tx.clone(),
        );

        let explorer = Explorer::new(ui_tx.clone());
        Self {
            selected_component: DashboardComponents::Sources,
            sources,
            accounts,
            explorer,
            ui_tx,
        }
    }
    fn change_selected_component(&mut self) {
        match self.selected_component {
            DashboardComponents::Sources => {
                self.selected_component = DashboardComponents::Accounts;
            }
            DashboardComponents::Accounts => {
                self.selected_component = DashboardComponents::Sources;
            }
            _ => {}
        }
    }
}

impl Component for Dashboard<'_> {
    fn render(&mut self, f: &mut ratatui::prelude::Frame, area: Rect, _: Option<ComponentProps>) {
        let [aside, main] = *Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(f.size())
        else {
            panic!("layout needs to have 2 chunks")
        };
        let [sources, accounts] = *Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(aside)
        else {
            panic!("aside should have 2 nested chunks")
        };
        self.sources.render(
            f,
            sources,
            Some(ComponentProps {
                selected: matches!(self.selected_component, DashboardComponents::Sources),
            }),
        );
        self.accounts.render(
            f,
            accounts,
            Some(ComponentProps {
                selected: matches!(self.selected_component, DashboardComponents::Accounts),
            }),
        );
        self.explorer.render(
            f,
            main,
            Some(ComponentProps {
                selected: matches!(self.selected_component, DashboardComponents::Explorer),
            }),
        );
    }

    fn handle_key_events(&mut self, key: KeyEvent) {
        match key.code {
            crossterm::event::KeyCode::Left | crossterm::event::KeyCode::Right => {
                self.change_selected_component()
            }
            _ => match self.selected_component {
                DashboardComponents::Sources => self.sources.handle_key_events(key),
                DashboardComponents::Accounts => self.accounts.handle_key_events(key),
                DashboardComponents::Explorer => self.explorer.handle_key_events(key),
            },
        }
    }
}

use core::panic;

use crossterm::event::KeyEvent;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use tokio::sync::mpsc::UnboundedSender;

use crate::{action::Action, store::state::AppState};

use super::{
    // accounts::Accounts,
    accounts::Accounts,
    component::{Component, ComponentProps},
    explorer::Explorer,
    sources::Sources, // sources::Sources,
};

pub struct Dashboard {
    selected_component: DashboardComponents,
    sources: Sources,
    accounts: Accounts,
    explorer: Explorer,
    ui_tx: UnboundedSender<Action>,
    aside_constraints: [Constraint; 2],
}

enum DashboardComponents {
    Sources,
    Accounts,
    Explorer,
}

impl Dashboard {
    pub fn new(state: &AppState, ui_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        let sources = Sources::new(&state.sources.available_sources, &None, ui_tx.clone());
        let accounts = Accounts::new(&state.accounts.available_accounts, &None, ui_tx.clone());

        let explorer = Explorer::new(None, ui_tx.clone());
        Dashboard {
            selected_component: DashboardComponents::Accounts,
            sources,
            accounts,
            explorer,
            ui_tx,
            aside_constraints: [Constraint::Length(3), Constraint::Fill(1)],
        }
    }
    pub fn refresh_components(self, state: &AppState) -> Self {
        let sources = Sources::new(
            &state.sources.available_sources,
            &state.sources.active_source,
            self.ui_tx.clone(),
        );
        let accounts = Accounts::new(
            &state.accounts.available_accounts,
            &state.accounts.active_account,
            self.ui_tx.clone(),
        );
        let explorer = Explorer::new(Some(state.explorer.file_tree.clone()), self.ui_tx.clone());
        Dashboard {
            selected_component: self.selected_component,
            sources,
            accounts,
            explorer,
            ui_tx: self.ui_tx,
            aside_constraints: self.aside_constraints,
        }
    }
    fn change_selected_component(&mut self) {
        match self.selected_component {
            DashboardComponents::Sources => {
                self.selected_component = DashboardComponents::Accounts;
                self.aside_constraints = [Constraint::Length(3), Constraint::Fill(1)]
            }
            DashboardComponents::Accounts => {
                self.selected_component = DashboardComponents::Sources;
                self.aside_constraints = [Constraint::Fill(1), Constraint::Length(3)]
            }
            _ => {}
        }
    }

    fn set_explorer_selected(&mut self) {
        self.selected_component = DashboardComponents::Explorer;
    }
    fn set_sources_selected(&mut self) {
        self.selected_component = DashboardComponents::Sources;
    }
}

impl Component for Dashboard {
    fn render(&mut self, f: &mut ratatui::prelude::Frame, _area: Rect, _: Option<ComponentProps>) {
        let [aside, main] = *Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(15), Constraint::Percentage(85)])
            .split(f.size())
        else {
            panic!("layout needs to have 2 chunks")
        };
        let [sources, accounts] = *Layout::default()
            .direction(Direction::Vertical)
            .constraints(self.aside_constraints)
            .split(aside)
        else {
            panic!("aside should have 2 nested chunks")
        };
        self.sources.render(
            f,
            sources,
            Some(ComponentProps {
                selected: matches!(
                    self.selected_component,
                    DashboardComponents::Sources | DashboardComponents::Explorer
                ),
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
            crossterm::event::KeyCode::Left
            | crossterm::event::KeyCode::Right
            | crossterm::event::KeyCode::Char('h')
            | crossterm::event::KeyCode::Char('l') => self.change_selected_component(),
            crossterm::event::KeyCode::Esc => match self.selected_component {
                DashboardComponents::Explorer => {
                    self.explorer.handle_key_events(key);
                    self.set_sources_selected();
                }
                DashboardComponents::Accounts => self.accounts.handle_key_events(key),
                DashboardComponents::Sources => self.sources.handle_key_events(key),
            },
            crossterm::event::KeyCode::Enter => match self.selected_component {
                DashboardComponents::Sources => {
                    self.sources.handle_key_events(key);
                    self.set_explorer_selected();
                }
                DashboardComponents::Accounts => self.accounts.handle_key_events(key),
                DashboardComponents::Explorer => self.explorer.handle_key_events(key),
            },
            _ => match self.selected_component {
                DashboardComponents::Sources => self.sources.handle_key_events(key),
                DashboardComponents::Accounts => self.accounts.handle_key_events(key),
                DashboardComponents::Explorer => self.explorer.handle_key_events(key),
            },
        }
    }
}

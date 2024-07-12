use core::panic;

use crossterm::event::KeyEvent;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use tokio::sync::mpsc::UnboundedSender;

use crate::{action::Action, logger::LOGGER, store::state::AppState};

use super::{
    accounts::Accounts,
    component::{Component, ComponentProps},
    explorer::Explorer,
    notifications::NotificationsUI,
    sources::Sources,
};

pub struct Dashboard {
    selected_component: DashboardComponents,
    previous_selected_component: DashboardComponents,
    sources: Sources,
    accounts: Accounts,
    explorer: Explorer,
    notifications: NotificationsUI,
    ui_tx: UnboundedSender<Action>,
    aside_constraints: [Constraint; 2],
}

#[derive(Clone)]
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
        let accounts = Accounts::new(
            &state.accounts.available_accounts,
            state.accounts.account_map.clone(),
            state.accounts.region.clone(),
            &None,
            ui_tx.clone(),
        );

        let explorer = Explorer::new(None, None, ui_tx.clone());
        let notifications = NotificationsUI::new(state.notifications.clone());
        Dashboard {
            selected_component: DashboardComponents::Accounts,
            previous_selected_component: DashboardComponents::Accounts,
            sources,
            accounts,
            explorer,
            notifications,
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
            state.accounts.account_map.clone(),
            state.accounts.region.clone(),
            &state.accounts.active_account,
            self.ui_tx.clone(),
        );
        let explorer = Explorer::new(
            Some(state.explorer.file_tree.clone()),
            state.explorer.selected_folder.clone(),
            self.ui_tx.clone(),
        );

        let notifications = NotificationsUI::new(state.notifications.clone());

        Dashboard {
            selected_component: self.selected_component,
            previous_selected_component: self.previous_selected_component,
            sources,
            accounts,
            explorer,
            notifications,
            ui_tx: self.ui_tx,
            aside_constraints: self.aside_constraints,
        }
    }
    fn change_selected_component(&mut self) {
        match self.selected_component {
            DashboardComponents::Sources => {
                self.previous_selected_component = DashboardComponents::Sources;
                self.selected_component = DashboardComponents::Accounts;
                self.aside_constraints = [Constraint::Length(3), Constraint::Fill(1)]
            }
            DashboardComponents::Accounts => {
                self.previous_selected_component = DashboardComponents::Accounts;
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
        let [content, notification_section] = *Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Length(3)])
            .split(main)
        else {
            panic!("layout needs to have 2 chunks")
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
            content,
            Some(ComponentProps {
                selected: matches!(self.selected_component, DashboardComponents::Explorer),
            }),
        );

        self.notifications.render(f, notification_section, None);
    }

    fn handle_key_events(&mut self, key: KeyEvent) {
        let keycode = key.code;
        match self.selected_component {
            DashboardComponents::Sources => match keycode {
                crossterm::event::KeyCode::Left
                | crossterm::event::KeyCode::Right
                | crossterm::event::KeyCode::Char('h')
                | crossterm::event::KeyCode::Char('l') => self.change_selected_component(),
                crossterm::event::KeyCode::Enter => {
                    self.sources.handle_key_events(key);
                    self.set_explorer_selected();
                }
                _ => self.sources.handle_key_events(key),
            },
            DashboardComponents::Explorer => match keycode {
                crossterm::event::KeyCode::Esc => {
                    self.explorer.handle_key_events(key);
                    self.set_sources_selected();
                }
                _ => self.explorer.handle_key_events(key),
            },
            DashboardComponents::Accounts => match keycode {
                crossterm::event::KeyCode::Left
                | crossterm::event::KeyCode::Right
                | crossterm::event::KeyCode::Char('h')
                | crossterm::event::KeyCode::Char('l')
                    if !self.accounts.is_locked() =>
                {
                    self.change_selected_component()
                }
                _ => self.accounts.handle_key_events(key),
            },
        };
    }
}

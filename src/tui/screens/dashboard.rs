use core::panic;

use crossterm::event::KeyEvent;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action,
    store::{
        notifications::types::Notification,
        sources::WithSources,
        state::{AppState, DashboardComponents},
    },
    tui::{
        components::traits::{Component, ComponentProps},
        sections::{
            accounts::Accounts, explorer::Explorer, notifications::NotificationsUI,
            sources::Sources,
        },
    },
};

pub struct Dashboard {
    selected_component: DashboardComponents,
    sources: Sources,
    accounts: Accounts,
    explorer: Explorer,
    notifications: NotificationsUI,
    ui_tx: UnboundedSender<Action>,
    aside_constraints: [Constraint; 2],
}

impl Dashboard {
    pub fn new(state: &AppState, ui_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        let sources = Sources::new(state.sources.get_available_sources(), &None, ui_tx.clone());
        let accounts = Accounts::new(
            &state.accounts.available_accounts,
            state.accounts.account_map.clone(),
            state.accounts.region.clone(),
            &None,
            ui_tx.clone(),
        );

        let explorer = Explorer::new(None, None, ui_tx.clone());
        let notifications = NotificationsUI::new(state.notifications.clone(), ui_tx.clone());
        Dashboard {
            selected_component: state.selected_component.clone(),
            sources,
            accounts,
            explorer,
            notifications,
            ui_tx,
            aside_constraints: [Constraint::Length(3), Constraint::Fill(1)],
        }
    }
    pub fn handle_alert(&mut self, alert: Notification) {
        self.notifications.set_alert(Some(alert));
    }
    pub fn refresh_components(self, state: &AppState) -> Self {
        let sources = Sources::new(
            state.sources.get_available_sources(),
            state.sources.get_active_source(),
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

        let notifications = self.notifications.refresh(state.notifications.clone());
        let aside_constraints =
            if matches!(&state.selected_component, &DashboardComponents::Accounts) {
                [Constraint::Length(3), Constraint::Fill(1)]
            } else {
                [Constraint::Fill(1), Constraint::Length(3)]
            };

        Dashboard {
            selected_component: state.selected_component.clone(),
            sources,
            accounts,
            explorer,
            notifications,
            ui_tx: self.ui_tx,
            aside_constraints,
        }
    }
    fn change_selected_component(&mut self) {
        let _ = self.ui_tx.send(Action::CycleSelectedComponent);
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
        if self.notifications.has_visible_alert() {
            return self.notifications.handle_key_events(key);
        }
        match self.selected_component {
            DashboardComponents::Sources => match keycode {
                crossterm::event::KeyCode::Left
                | crossterm::event::KeyCode::Right
                | crossterm::event::KeyCode::Char('h')
                | crossterm::event::KeyCode::Char('l') => self.change_selected_component(),
                crossterm::event::KeyCode::Enter => {
                    self.sources.handle_key_events(key);
                }
                _ => self.sources.handle_key_events(key),
            },
            DashboardComponents::Explorer => match keycode {
                crossterm::event::KeyCode::Esc => {
                    self.explorer.handle_key_events(key);
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

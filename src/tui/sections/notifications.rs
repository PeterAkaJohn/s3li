use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action,
    store::notifications::{
        types::{Notification, NotificationType},
        Notifications,
    },
    tui::components::traits::{Component, ComponentProps},
};

pub struct NotificationsUI {
    notifications: Notifications,
    alert: Option<Notification>,
    ui_tx: UnboundedSender<Action>,
}

impl NotificationsUI {
    pub fn new(notifications: Notifications, ui_tx: UnboundedSender<Action>) -> NotificationsUI {
        NotificationsUI {
            notifications,
            ui_tx,
            alert: None,
        }
    }
    pub fn refresh(mut self, notifications: Notifications) -> Self {
        self.notifications = notifications;
        self
    }
    pub fn has_visible_alert(&self) -> bool {
        self.alert.is_some()
    }

    pub fn set_alert(&mut self, alert: Option<Notification>) {
        self.alert = alert;
    }
}

impl Component for NotificationsUI {
    fn render(
        &mut self,
        f: &mut ratatui::prelude::Frame,
        area: ratatui::prelude::Rect,
        _props: Option<ComponentProps>,
    ) {
        let container = Block::default()
            .borders(Borders::ALL)
            .title("Notifications")
            .border_type(BorderType::Rounded);
        if let Some(alert) = &self.alert {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Fill(1),
                    Constraint::Fill(2),
                    Constraint::Fill(1),
                ])
                .split(f.size());
            let center_section = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Fill(2),
                    Constraint::Fill(1),
                    Constraint::Fill(2),
                ])
                .margin(1)
                .split(layout[1])[1];

            let notification_text = Paragraph::new(alert.get_message())
                .block(container.clone())
                .wrap(Wrap::default())
                .style(Style::default().fg(Color::Red));
            f.render_widget(Clear, center_section);
            f.render_widget(notification_text, center_section);
        }
        if let Some(NotificationType::Notification(notification)) = self.notifications.get_last() {
            let inner_container = container.inner(area);
            let style = if notification.has_error() {
                Style::default().fg(Color::Red)
            } else {
                Style::default().fg(Color::LightGreen)
            };
            let notification_text = Paragraph::new(notification.get_message()).style(style);
            f.render_widget(notification_text, inner_container);
        };
        f.render_widget(container, area);
    }
    fn handle_key_events(&mut self, _key: crossterm::event::KeyEvent) {
        self.alert = None;
        let _ = self.ui_tx.send(Action::DismissLastAlert);
    }
}

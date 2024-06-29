use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::store::notifications::Notifications;

use super::component::{Component, ComponentProps, WithContainer};

pub struct NotificationsUI {
    notifications: Notifications,
}

impl NotificationsUI {
    pub fn new(notifications: Notifications) -> NotificationsUI {
        NotificationsUI { notifications }
    }
}

impl WithContainer<'_> for NotificationsUI {}

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
        if let Some(notification) = self.notifications.get_last() {
            let inner_container = container.inner(area);
            let style = if notification.error {
                Style::default().fg(Color::Red)
            } else {
                Style::default().fg(Color::LightGreen)
            };
            let notification_text = Paragraph::new(notification.message.clone()).style(style);
            f.render_widget(notification_text, inner_container);
        }
        f.render_widget(container, area);
    }
    fn handle_key_events(&mut self, _key: crossterm::event::KeyEvent) {}
}

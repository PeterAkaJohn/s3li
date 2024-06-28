use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, Padding, Paragraph},
};

use crate::store::notifications::Notifications;

use super::component::{Component, ComponentProps};

pub struct NotificationsUI {
    notifications: Notifications,
}

impl NotificationsUI {
    pub fn new(notifications: Notifications) -> NotificationsUI {
        NotificationsUI { notifications }
    }
}

impl Component for NotificationsUI {
    fn render(
        &mut self,
        f: &mut ratatui::prelude::Frame,
        area: ratatui::prelude::Rect,
        _props: Option<ComponentProps>,
    ) {
        if let Some(notification) = self.notifications.get_last() {
            let style = if notification.error {
                Style::default().fg(Color::Red)
            } else {
                Style::default().fg(Color::LightGreen)
            };
            let notification_text = Paragraph::new(notification.message.clone())
                .block(Block::default().padding(Padding::horizontal(1)))
                .style(style);
            f.render_widget(notification_text, area);
        }
    }
    fn handle_key_events(&mut self, _key: crossterm::event::KeyEvent) {}
}

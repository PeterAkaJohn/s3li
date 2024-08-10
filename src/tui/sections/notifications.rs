use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Styled},
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap},
};

use crate::{
    store::notifications::{
        types::{self, NotificationType},
        Notifications,
    },
    tui::components::traits::{Component, ComponentProps, WithContainer},
};

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
            match notification {
                NotificationType::Notification(notification) => {
                    let inner_container = container.inner(area);
                    let style = if notification.has_error() {
                        Style::default().fg(Color::Red)
                    } else {
                        Style::default().fg(Color::LightGreen)
                    };
                    let notification_text = Paragraph::new(notification.get_message()).style(style);
                    f.render_widget(notification_text, inner_container);
                }
                NotificationType::Alert(notification) => {
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

                    let notification_text = Paragraph::new(notification.get_message())
                        .block(container.clone())
                        .wrap(Wrap::default())
                        .style(Style::default().fg(Color::Red));
                    f.render_widget(Clear, center_section);
                    f.render_widget(notification_text, center_section);
                }
            };
        }
        f.render_widget(container, area);
    }
    fn handle_key_events(&mut self, _key: crossterm::event::KeyEvent) {}
}

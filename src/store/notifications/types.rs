#[derive(Default, Debug, Clone)]
pub struct Notification {
    message: String,
    error: bool,
    shown: bool,
}

impl Notification {
    pub fn get_message(&self) -> &str {
        &self.message
    }
    pub fn has_error(&self) -> bool {
        self.error
    }
    pub fn has_been_shown(&self) -> bool {
        self.shown
    }
    pub fn set_shown(&mut self) {
        self.shown = true
    }
}

#[derive(Default, Debug, Clone)]
pub struct Notifications {
    notifications: Vec<NotificationType>,
}

impl Notifications {
    pub fn push_notification(&mut self, message: String, error: bool) {
        self.notifications
            .push(NotificationType::Notification(Notification {
                message,
                error,
                shown: true,
            }));
    }

    pub fn push_alert(&mut self, message: String) {
        self.notifications
            .push(NotificationType::Alert(Notification {
                message,
                error: true,
                shown: false,
            }));
    }

    pub fn get_last(&self) -> Option<&NotificationType> {
        self.notifications.last()
    }
    pub fn set_last_alert_as_shown(&mut self) {
        if let Some(NotificationType::Alert(notification)) = self.notifications.last_mut() {
            notification.set_shown();
        }
    }
}

#[derive(Clone, Debug)]
pub enum NotificationType {
    Notification(Notification),
    Alert(Notification),
}

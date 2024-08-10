#[derive(Default, Debug, Clone)]
pub struct Notification {
    message: String,
    error: bool,
}

impl Notification {
    pub fn get_message(&self) -> &str {
        &self.message
    }
    pub fn has_error(&self) -> bool {
        self.error
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
            }));
    }

    pub fn push_alert(&mut self, message: String) {
        self.notifications
            .push(NotificationType::Alert(Notification {
                message,
                error: true,
            }));
    }

    pub fn get_last(&self) -> Option<&NotificationType> {
        self.notifications.last()
    }
}

#[derive(Clone, Debug)]
pub enum NotificationType {
    Notification(Notification),
    Alert(Notification),
}

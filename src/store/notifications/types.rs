#[derive(Default, Debug, Clone)]
pub struct Notification {
    pub message: String,
    pub error: bool,
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

    pub fn push_alert(&mut self, message: String, error: bool) {
        self.notifications
            .push(NotificationType::Alert(Notification { message, error }));
    }

    pub fn get_last(&self) -> Option<&Notification> {
        match self.notifications.last() {
            Some(notification) => match notification {
                NotificationType::Notification(notification) => Some(notification),
                NotificationType::Alert(notification) => Some(notification),
            },
            None => None,
        }
    }
}

#[derive(Clone, Debug)]
enum NotificationType {
    Notification(Notification),
    Alert(Notification),
}

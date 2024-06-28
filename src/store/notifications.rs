#[derive(Default, Debug, Clone)]
pub struct Notification {
    pub message: String,
    pub error: bool,
}

#[derive(Default, Debug, Clone)]
pub struct Notifications {
    notifications: Vec<Notification>,
}

impl Notifications {
    pub fn push(&mut self, message: String, error: bool) {
        self.notifications.push(Notification { message, error });
    }

    pub fn get_last(&self) -> Option<&Notification> {
        return self.notifications.last();
    }
}

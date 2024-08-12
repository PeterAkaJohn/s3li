use crate::store::{
    accounts::Accounts,
    action_manager::ActionManager,
    explorer::Explorer,
    notifications::{types::Notification, Notifications},
    sources::Sources,
};

#[derive(Default, Debug, Clone)]
pub enum DashboardComponents {
    Sources,
    #[default]
    Accounts,
    Explorer,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub sources: Sources,
    pub accounts: Accounts,
    pub explorer: Explorer,
    pub action_manager: ActionManager,
    pub notifications: Notifications,
    pub selected_component: DashboardComponents,
}

pub enum StateEvents {
    UpdateState(AppState),
    Alert(Notification),
}

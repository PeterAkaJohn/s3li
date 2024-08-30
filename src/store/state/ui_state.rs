use crate::{
    providers::AccountMap,
    store::{
        accounts::Accounts,
        action_manager::ActionManager,
        explorer::{Explorer, FileTree, Folder},
        notifications::Notifications,
        sources::{Sources, WithSources},
    },
};

use super::{AppState, DashboardComponents};

pub struct UISources {
    pub available_sources: Vec<String>,
    pub active_source: Option<String>,
}

impl From<Sources> for UISources {
    fn from(value: Sources) -> Self {
        Self {
            available_sources: value.get_available_sources().clone(),
            active_source: value.get_active_source().clone(),
        }
    }
}

pub struct UIExplorer {
    pub selected_folder: Option<Folder>,
    pub file_tree: FileTree,
}

impl From<Explorer> for UIExplorer {
    fn from(value: Explorer) -> Self {
        Self {
            selected_folder: value.selected_folder,
            file_tree: value.file_tree,
        }
    }
}
pub struct UIAccounts {
    pub account_map: AccountMap,
    pub available_accounts: Vec<String>,
    pub active_account: Option<String>,
    pub region: String,
}

impl From<Accounts> for UIAccounts {
    fn from(value: Accounts) -> Self {
        Self {
            account_map: value.account_map,
            available_accounts: value.available_accounts,
            active_account: value.active_account,
            region: value.region,
        }
    }
}
pub struct UIState {
    pub sources: UISources,
    pub explorer: UIExplorer,
    pub accounts: UIAccounts,
    pub notifications: Notifications,
    pub selected_component: DashboardComponents,
    pub action_manager: ActionManager,
}

impl From<AppState> for UIState {
    fn from(value: AppState) -> Self {
        Self {
            sources: value.sources.into(),
            explorer: value.explorer.into(),
            accounts: value.accounts.into(),
            notifications: value.notifications,
            selected_component: value.selected_component,
            action_manager: value.action_manager,
        }
    }
}

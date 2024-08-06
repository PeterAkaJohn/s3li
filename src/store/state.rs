use std::sync::Arc;

use anyhow::Result;
use tokio::sync::{
    mpsc::{self, UnboundedReceiver, UnboundedSender},
    Mutex,
};

use crate::{
    action::Action,
    logger::{LogToFile, LOGGER},
    providers::AwsClient,
};

use super::{
    accounts::Accounts,
    action_manager::ActionManager,
    explorer::{Explorer, File, Folder, TreeItem},
    notifications::Notifications,
    sources::{
        buckets::{entities::BucketItem, Buckets},
        traits::WithSources,
        Sources,
    },
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

impl LogToFile for AppState {
    fn info(&self, message: &str) -> Result<()> {
        let _ = self.write_to_file(message);
        self.write_to_file(&format!("{:#?}", self))
    }
}

pub struct State {
    pub app_state: AppState,
    pub tx: UnboundedSender<AppState>,
}

impl State {
    pub async fn new(client: Arc<Mutex<AwsClient>>) -> (Self, UnboundedReceiver<AppState>) {
        let (tx, rx) = mpsc::unbounded_channel();
        let app_state = AppState {
            sources: Sources::Buckets(Buckets::new(client.clone())),
            explorer: Explorer::new(client.clone()),
            accounts: Accounts::new(client.clone(), None).await,
            action_manager: ActionManager::default(),
            notifications: Notifications::default(),
            selected_component: DashboardComponents::default(),
        };
        (Self { tx, app_state }, rx)
    }

    async fn handle_state_action(&mut self, action: Action) {
        match action {
            Action::SetExplorerFolder(tree_item) => {
                let new_selected_folder = self
                    .app_state
                    .explorer
                    .update_file_tree(
                        self.app_state.sources.get_active_source().as_ref().unwrap(),
                        &tree_item,
                    )
                    .await;
                if let Some(folder) = new_selected_folder {
                    self.app_state
                        .notifications
                        .push(format!("Folder {} has been selected", folder.name), false);
                }
            }
            Action::SetSource(source_idx) => {
                let bucket = self.app_state.sources.set_source_with_idx(source_idx);
                if let Some(bucket) = bucket {
                    self.app_state.explorer.create_file_tree(bucket).await;
                    self.app_state.selected_component = DashboardComponents::Explorer;
                    self.app_state
                        .notifications
                        .push(format!("Source {bucket} has been selected"), false);
                }
            }
            Action::SetAccount(account_idx) => {
                self.app_state.accounts.set_account(account_idx).await;
                self.app_state.sources.update_available_sources().await;
                self.app_state.notifications.push(
                    format!("Account with idx {account_idx} has been selected"),
                    false,
                );
            }
            Action::ChangeRegion(new_region) => {
                self.app_state
                    .accounts
                    .change_region(new_region.clone())
                    .await;
                self.app_state
                    .notifications
                    .push(format!("Region changed to {}", &new_region), false);
            }
            Action::RefreshCredentials => {
                self.app_state.accounts.refresh_credentials().await;
                self.app_state
                    .notifications
                    .push("Credentials refreshed".to_string(), false);
            }

            Action::EditCredentials(account, properties) => {
                self.app_state
                    .accounts
                    .edit_credentials(account, properties)
                    .await;
                self.app_state
                    .notifications
                    .push("Credentials updated".to_string(), false);
            }
            Action::Download(items_to_download) => {
                let items: Vec<BucketItem> = items_to_download
                    .into_iter()
                    .map(|item| item.into())
                    .collect();
                if let Err(e) = self.app_state.sources.download(items).await {
                    let _ = LOGGER.info("error downloading items");
                    let _ = LOGGER.info(&format!("{:?}", e));
                    self.app_state
                        .notifications
                        .push("Failed to download items".to_string(), true);
                } else {
                    self.app_state
                        .notifications
                        .push("Successfully downloaded requested items".to_string(), false);
                }
            }
            Action::CycleSelectedComponent => match self.app_state.selected_component {
                DashboardComponents::Sources => {
                    self.app_state.selected_component = DashboardComponents::Accounts;
                }
                DashboardComponents::Accounts => {
                    self.app_state.selected_component = DashboardComponents::Sources;
                }
                _ => {}
            },
            Action::SetSelectedComponent(selected_component) => {
                self.app_state.selected_component = selected_component;
            }
            unhandled_action => {
                let _ = LOGGER.info(&format!("ignoring action {:#?}", unhandled_action));
            }
        };
    }

    pub async fn start(&mut self, mut ui_rx: UnboundedReceiver<Action>) -> Result<()> {
        // we need to send first state to unlock the ui
        self.tx.send(self.app_state.clone())?;
        // need to loop over ui_rx to react to user input
        loop {
            tokio::select! {
                Some(action) = ui_rx.recv() => {
                    self.app_state.action_manager.push(action.clone());
                    match action {
                        Action::Quit => break Ok(()),
                        Action::Tick => {},
                        Action::Render => {},
                        Action::Key(_) =>{},
                        _ => self.handle_state_action(action).await,
                    };
                    self.tx.send(self.app_state.clone())?;
                }
            }
        }
    }
}

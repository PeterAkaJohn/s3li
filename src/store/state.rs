mod app_state;
pub use app_state::StateEvents;
pub use app_state::{AppState, DashboardComponents};

use std::sync::Arc;

use anyhow::Result;
use tokio::sync::{
    mpsc::{self, UnboundedReceiver, UnboundedSender},
    Mutex,
};

use crate::store::notifications::types::NotificationType;
use crate::{action::Action, logger::LOGGER, providers::AwsClient};

use super::{
    accounts::Accounts,
    action_manager::ActionManager,
    explorer::Explorer,
    notifications::Notifications,
    sources::{
        buckets::{entities::BucketItem, Buckets},
        traits::WithSources,
        Sources,
    },
};

pub struct State {
    pub app_state: AppState,
    pub tx: UnboundedSender<StateEvents>,
}

impl State {
    pub async fn new(client: Arc<Mutex<AwsClient>>) -> (Self, UnboundedReceiver<StateEvents>) {
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
                    self.app_state.notifications.push_notification(
                        format!("Folder {} has been selected", folder.name),
                        false,
                    );
                }
            }
            Action::SetSource(source_idx) => {
                let bucket = self.app_state.sources.set_source_with_idx(source_idx);
                if let Some(bucket) = bucket {
                    self.app_state.explorer.create_file_tree(bucket).await;
                    self.app_state.selected_component = DashboardComponents::Explorer;
                    self.app_state
                        .notifications
                        .push_notification(format!("Source {bucket} has been selected"), false);
                }
            }
            Action::SetAccount(account_idx) => {
                self.app_state.accounts.set_account(account_idx).await;
                self.app_state.sources.update_available_sources().await;
                self.app_state.notifications.push_notification(
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
                    .push_notification(format!("Region changed to {}", &new_region), false);
            }
            Action::RefreshCredentials => {
                self.app_state.accounts.refresh_credentials().await;
                self.app_state
                    .notifications
                    .push_notification("Credentials refreshed".to_string(), false);
            }

            Action::EditCredentials(account, properties) => {
                self.app_state
                    .accounts
                    .edit_credentials(account, properties)
                    .await;
                self.app_state
                    .notifications
                    .push_notification("Credentials updated".to_string(), false);
            }
            Action::Download(items_to_download) => {
                let items: Vec<BucketItem> = items_to_download
                    .into_iter()
                    .map(|item| item.into())
                    .collect();
                let download_result = self.app_state.sources.download(items).await;

                let _ = LOGGER.info(&format!("download result {download_result:#?}"));

                if download_result.results.iter().any(|(_, res)| res.is_err()) {
                    let mut failed_items = vec![];
                    for res in download_result.results {
                        match res {
                            (file_key, Ok(_)) => {
                                self.app_state.notifications.push_notification(
                                    format!("Successfully downloaded requested item {file_key}"),
                                    false,
                                );
                            }
                            (file_key, Err(e)) => {
                                let _ = LOGGER.info(&format!("error downloading item {file_key}"));
                                let _ = LOGGER.info(&format!("{:?}", e));
                                failed_items.push(file_key);
                                // self.app_state.notifications.push_notification(
                                //     format!("Failed to download item {file_key}"),
                                // //     true,
                                // );
                            }
                        }
                    }
                    let mut alert_message = "These items failed downloading:".to_string();
                    for item in &failed_items {
                        alert_message.push_str(&format!("\n{item}"));
                    }
                    self.app_state.notifications.push_alert(alert_message);
                } else {
                    self.app_state.notifications.push_notification(
                        "Successfully downloaded requested items".to_string(),
                        false,
                    );
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
            Action::DismissLastAlert => {
                self.app_state.notifications.set_last_alert_as_shown();
            }
            unhandled_action => {
                let _ = LOGGER.info(&format!("ignoring action {:#?}", unhandled_action));
            }
        };
    }

    pub async fn start(&mut self, mut ui_rx: UnboundedReceiver<Action>) -> Result<()> {
        // we need to send first state to unlock the ui
        self.tx
            .send(StateEvents::UpdateState(self.app_state.clone()))?;
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
                let last_notification = self.app_state.notifications.get_last();
                if let Some(NotificationType::Alert(alert)) = last_notification {
                        if !alert.has_been_shown() {
                            self.tx.send(StateEvents::Alert(alert.clone()))?;
                        }
                    }
                self.tx
                    .send(StateEvents::UpdateState(self.app_state.clone()))?;
                }
            }
        }
    }
}

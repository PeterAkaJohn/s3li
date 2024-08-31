use anyhow::Result;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action,
    logger::LOGGER,
    store::{
        accounts::Accounts,
        action_manager::ActionManager,
        explorer::Explorer,
        notifications::{types::NotificationType, Notifications},
        sources::{buckets::entities::BucketItem, Sources, WithSources},
    },
};

use super::StateEvents;

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

impl DashboardComponents {
    fn default_actions(&self, app_state: &mut AppState, action: &Action) {
        match action {
            Action::CycleSelectedComponent => match app_state.selected_component {
                DashboardComponents::Sources => {
                    app_state.selected_component = DashboardComponents::Accounts;
                }
                DashboardComponents::Accounts => {
                    app_state.selected_component = DashboardComponents::Sources;
                }
                _ => {}
            },
            Action::SetSelectedComponent(selected_component) => {
                app_state.selected_component = selected_component.clone();
            }
            Action::DismissLastAlert => {
                app_state.notifications.set_last_alert_as_shown();
            }
            unhandled_action => {
                let _ = LOGGER.info(&format!("ignoring action {:#?}", unhandled_action));
            }
        }
    }
    async fn handle_sources_action(&self, app_state: &mut AppState, action: &Action) {
        match action {
            Action::SetSource(source_idx) => {
                let bucket = app_state.sources.set_source_with_idx(*source_idx);
                if let Some(bucket) = bucket {
                    app_state.explorer.create_file_tree(bucket).await;
                    app_state.selected_component = DashboardComponents::Explorer;
                    app_state
                        .notifications
                        .push_notification(format!("Source {bucket} has been selected"), false);
                }
            }
            unhandled_action => self.default_actions(app_state, unhandled_action),
        }
    }

    async fn handle_accounts_actions(&self, app_state: &mut AppState, action: &Action) {
        match action {
            Action::SetAccount(account_idx) => {
                let account = app_state.accounts.set_account(*account_idx).await;
                match app_state.sources.update_available_sources().await {
                    Ok(_) => {
                        app_state.notifications.push_notification(
                            format!("Account {account} has been selected"),
                            false,
                        );
                    }
                    Err(_) => {
                        app_state
                            .notifications
                            .push_alert(format!("Failed to set account {account}"));
                    }
                }
            }
            Action::ChangeRegion(new_region) => {
                app_state.accounts.change_region(new_region.clone()).await;
                app_state
                    .notifications
                    .push_notification(format!("Region changed to {}", &new_region), false);
            }
            Action::RefreshCredentials => match app_state.accounts.refresh_credentials().await {
                Ok(_) => {
                    app_state
                        .notifications
                        .push_notification("Credentials refreshed".to_string(), false);
                }
                Err(_) => {
                    app_state.notifications.push_notification(
                        "Failed to refresh credentials. Try again.".to_string(),
                        true,
                    );
                }
            },

            Action::EditCredentials(account, properties) => {
                match app_state
                    .accounts
                    .edit_credentials(account.clone(), properties.clone())
                    .await
                {
                    Ok(_) => {
                        app_state
                            .notifications
                            .push_notification("Credentials updated".to_string(), false);
                    }
                    Err(_) => {
                        app_state.notifications.push_alert(format!(
                            "Failed to update credentials for account {account}"
                        ));
                    }
                }
            }
            unhandled_action => self.default_actions(app_state, unhandled_action),
        }
    }

    async fn handle_explorer_actions(&self, app_state: &mut AppState, action: &Action) {
        match action {
            Action::SetExplorerFolder(tree_item) => {
                let new_selected_folder = app_state
                    .explorer
                    .update_file_tree(
                        app_state.sources.get_active_source().as_ref().unwrap(),
                        tree_item,
                    )
                    .await;
                if let Some(folder) = new_selected_folder {
                    app_state.notifications.push_notification(
                        format!("Folder {} has been selected", folder.name),
                        false,
                    );
                }
            }
            Action::Download(items_to_download) => {
                let items: Vec<BucketItem> = items_to_download
                    .iter()
                    .map(|item| item.clone().into())
                    .collect();
                let download_result = app_state.sources.download(items).await;

                let _ = LOGGER.info(&format!("download result {download_result:#?}"));

                if download_result.results.iter().any(|(_, res)| res.is_err()) {
                    let mut failed_items = vec![];
                    for res in download_result.results {
                        match res {
                            (file_key, Ok(_)) => {
                                app_state.notifications.push_notification(
                                    format!("Successfully downloaded requested item {file_key}"),
                                    false,
                                );
                            }
                            (file_key, Err(e)) => {
                                let _ = LOGGER.info(&format!("error downloading item {file_key}"));
                                let _ = LOGGER.info(&format!("{:?}", e));
                                failed_items.push(file_key);
                            }
                        }
                    }
                    let mut alert_message = "These items failed downloading:".to_string();
                    for item in &failed_items {
                        alert_message.push_str(&format!("\n{item}"));
                    }
                    app_state.notifications.push_alert(alert_message);
                } else {
                    app_state.notifications.push_notification(
                        "Successfully downloaded requested items".to_string(),
                        false,
                    );
                }
            }
            unhandled_action => self.default_actions(app_state, unhandled_action),
        }
    }

    async fn handle_action(&self, mut app_state: AppState, action: &Action) -> AppState {
        match self {
            DashboardComponents::Sources => {
                self.handle_sources_action(&mut app_state, action).await
            }
            DashboardComponents::Accounts => {
                self.handle_accounts_actions(&mut app_state, action).await
            }
            DashboardComponents::Explorer => {
                self.handle_explorer_actions(&mut app_state, action).await
            }
        }
        app_state
    }
}

impl AppState {
    pub async fn handle_state_action(
        &mut self,
        action: Action,
        tx: UnboundedSender<StateEvents>,
    ) -> Result<Self> {
        let app_state = self
            .selected_component
            .handle_action(self.clone(), &action)
            .await;

        let last_notification = app_state.notifications.get_last();
        if let Some(NotificationType::Alert(alert)) = last_notification {
            if !alert.has_been_shown() {
                tx.send(StateEvents::Alert(alert.clone()))?;
            }
        }
        tx.send(StateEvents::UpdateState(app_state.clone().into()))?;
        Ok(app_state)
    }
}

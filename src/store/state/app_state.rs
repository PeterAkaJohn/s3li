use crate::{
    action::Action,
    logger::LOGGER,
    store::{
        accounts::Accounts,
        action_manager::ActionManager,
        explorer::Explorer,
        notifications::Notifications,
        sources::{buckets::entities::BucketItem, Sources, WithSources},
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

impl AppState {
    pub async fn handle_state_action(&mut self, action: Action) {
        match action {
            Action::SetExplorerFolder(tree_item) => {
                let new_selected_folder = self
                    .explorer
                    .update_file_tree(
                        self.sources.get_active_source().as_ref().unwrap(),
                        &tree_item,
                    )
                    .await;
                if let Some(folder) = new_selected_folder {
                    self.notifications.push_notification(
                        format!("Folder {} has been selected", folder.name),
                        false,
                    );
                }
            }
            Action::SetSource(source_idx) => {
                let bucket = self.sources.set_source_with_idx(source_idx);
                if let Some(bucket) = bucket {
                    self.explorer.create_file_tree(bucket).await;
                    self.selected_component = DashboardComponents::Explorer;
                    self.notifications
                        .push_notification(format!("Source {bucket} has been selected"), false);
                }
            }
            Action::SetAccount(account_idx) => {
                self.accounts.set_account(account_idx).await;
                self.sources.update_available_sources().await;
                self.notifications.push_notification(
                    format!("Account with idx {account_idx} has been selected"),
                    false,
                );
            }
            Action::ChangeRegion(new_region) => {
                self.accounts.change_region(new_region.clone()).await;
                self.notifications
                    .push_notification(format!("Region changed to {}", &new_region), false);
            }
            Action::RefreshCredentials => {
                self.accounts.refresh_credentials().await;
                self.notifications
                    .push_notification("Credentials refreshed".to_string(), false);
            }

            Action::EditCredentials(account, properties) => {
                self.accounts.edit_credentials(account, properties).await;
                self.notifications
                    .push_notification("Credentials updated".to_string(), false);
            }
            Action::Download(items_to_download) => {
                let items: Vec<BucketItem> = items_to_download
                    .into_iter()
                    .map(|item| item.into())
                    .collect();
                let download_result = self.sources.download(items).await;

                let _ = LOGGER.info(&format!("download result {download_result:#?}"));

                if download_result.results.iter().any(|(_, res)| res.is_err()) {
                    let mut failed_items = vec![];
                    for res in download_result.results {
                        match res {
                            (file_key, Ok(_)) => {
                                self.notifications.push_notification(
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
                    self.notifications.push_alert(alert_message);
                } else {
                    self.notifications.push_notification(
                        "Successfully downloaded requested items".to_string(),
                        false,
                    );
                }
            }
            Action::CycleSelectedComponent => match self.selected_component {
                DashboardComponents::Sources => {
                    self.selected_component = DashboardComponents::Accounts;
                }
                DashboardComponents::Accounts => {
                    self.selected_component = DashboardComponents::Sources;
                }
                _ => {}
            },
            Action::SetSelectedComponent(selected_component) => {
                self.selected_component = selected_component;
            }
            Action::DismissLastAlert => {
                self.notifications.set_last_alert_as_shown();
            }
            unhandled_action => {
                let _ = LOGGER.info(&format!("ignoring action {:#?}", unhandled_action));
            }
        };
    }
}

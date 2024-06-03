use core::panic;

use anyhow::Result;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::{
    action::Action,
    logger::{LogToFile, LOGGER},
    providers::AwsClient,
    store::explorer::TreeItem,
};

use super::{
    accounts::Accounts,
    explorer::{Explorer, FileTree},
    sources::Sources,
};

#[derive(Debug, Default, Clone)]
pub struct AppState {
    pub sources: Sources,
    pub accounts: Accounts,
    pub explorer: Explorer,
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
    pub client: AwsClient,
}

impl State {
    pub async fn new() -> (Self, UnboundedReceiver<AppState>) {
        let (tx, rx) = mpsc::unbounded_channel();
        let client = AwsClient::new().await;
        let mut accounts = AwsClient::list_accounts();
        accounts.sort();
        let app_state = AppState {
            sources: Sources {
                available_sources: vec![],
                active_source: None,
            },
            explorer: Explorer::new(),
            accounts: Accounts {
                active_account: None,
                available_accounts: accounts,
            },
        };
        (
            Self {
                tx,
                app_state,
                client,
            },
            rx,
        )
    }

    pub async fn start(&mut self, mut ui_rx: UnboundedReceiver<Action>) -> Result<()> {
        // we need to send first state to unlock the ui
        self.tx.send(self.app_state.clone())?;
        // need to loop over ui_rx to react to user input
        loop {
            tokio::select! {
                Some(action) = ui_rx.recv() => {
                    match action {
                        Action::Quit => break Ok(()),
                        Action::Tick => {},
                        Action::Render => {},
                        Action::Key(_) =>{},
                        Action::SetExplorerFolder(tree_item) => {
                            let new_selected_folder = if let TreeItem::Folder(folder,parent) = tree_item {
                                if self.app_state.explorer.selected_folder == Some(folder.clone()) {
                                    // this is means that we need to remove child of selected
                                    // folder so we return the parent
                                    if let Some(parent_folder) = parent {
                                        if parent_folder.name == "/" {
                                            None
                                        } else {
                                            Some(parent_folder)
                                        }
                                    } else {
                                        None
                                    }
                                } else {
                                    Some(folder)
                                }
                            } else {
                                panic!("cannot be a file tree_item");
                            };
                            LOGGER.info(&format!("{:?}", new_selected_folder));
                            let (files,folders) = self.client.list_objects(&self.app_state.sources.active_source.clone().unwrap(), new_selected_folder.clone().map(|folder| folder.name).as_deref()).await;
                            if let Some(folder) = new_selected_folder.clone() {
                                self.app_state.explorer
                                    .update_folder(folder, files.iter().map(|file_key| file_key.parse().expect("file creation cannot fail")).collect(), folders.iter().map(|new_folder| new_folder.parse().expect("folder creation cannot fail")).collect());
                            } else {
                                let file_tree = FileTree::new(
                                    "/".parse().expect("root_folder initialization cannot fail"),
                                    folders.iter().map(|folder| folder.parse().expect("folder creation cannot fail")).collect(),
                                    files.iter().map(|file_key| file_key.parse().expect("file creation cannot fail")).collect());
                                self.app_state.explorer.file_tree = file_tree;
                            }
                            self.app_state.explorer.selected_folder = new_selected_folder;
                            self.tx.send(self.app_state.clone())?;
                        },
                        Action::SetSource(source_idx) => {
                            let bucket = self.app_state.sources.available_sources.get(source_idx).map(|val| val.to_string());
                            self.app_state.sources.active_source = bucket.clone();
                            let (files,folders) = self.client.list_objects(&bucket.clone().unwrap(), None).await;
                            let file_tree = FileTree::new(
                                "/".parse().expect("root_folder initialization cannot fail"),
                                folders.iter().map(|folder| folder.parse().expect("folder creation cannot fail")).collect(),
                                files.iter().map(|file_key| file_key.parse().expect("file creation cannot fail")).collect());
                            self.app_state.explorer.selected_folder = Some("/".parse().expect("root_folder initialization cannot fail"));
                            self.app_state.explorer.file_tree = file_tree;
                            self.tx.send(self.app_state.clone())?;
                        },
                        Action::SetAccount(account_idx) => {
                            let account = self.app_state.accounts.available_accounts.get(account_idx).map(|val| val.as_str()).unwrap_or("default");
                            self.app_state.accounts.active_account = Some(account.to_string());
                            self.client.switch_account(account).await;
                            let buckets = self.client.list_buckets().await;
                            self.app_state.sources.available_sources = if let Ok(buckets) = buckets {buckets} else {vec![]};
                            self.tx.send(self.app_state.clone())?;
                        },
                    }
                }
            }
        }
    }
}

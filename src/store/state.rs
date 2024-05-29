use anyhow::Result;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::{
    action::Action,
    providers::AwsClient,
    store::explorer::{File, Folder},
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
                        Action::SetExplorerFolder(folder) => {
                            let (files,folders) = self.client.list_objects(self.app_state.sources.active_source.clone().unwrap(), Some(&folder)).await;
                            let new_selected_folder = Folder{name: folder.clone()};
                            self.app_state.explorer.update_folder(new_selected_folder, files.iter().map(|file_key| File{name: file_key.to_owned()}).collect(), folders.iter().map(|new_folder| Folder{name: new_folder.to_owned()}).collect());
                            self.app_state.explorer.selected_folder = Some(Folder{name: folder.clone()});
                            self.tx.send(self.app_state.clone())?;
                        },
                        Action::SetSource(source_idx) => {
                            let bucket = self.app_state.sources.available_sources.get(source_idx).map(|val| val.to_string());
                            self.app_state.sources.active_source = bucket.clone();
                            let (files,folders) = self.client.list_objects(bucket.clone().unwrap(), None).await;
                            let file_tree = FileTree::new(Folder{name: "/".to_string()}, folders.iter().map(|file_key| Folder{name: file_key.to_owned()}).collect() ,files.iter().map(|file_key| File{name: file_key.to_owned()}).collect());
                            self.app_state.explorer.selected_folder = Some(Folder{name: "/".to_string()});
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

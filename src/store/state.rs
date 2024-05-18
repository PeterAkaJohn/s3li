use anyhow::Result;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::{action::Action, providers::AwsClient};

#[derive(Debug, Default, Clone)]
pub struct Sources {
    pub available_sources: Vec<String>,
    pub active_source: Option<String>,
}
#[derive(Debug, Default, Clone)]
pub struct Accounts {
    pub available_accounts: Vec<String>,
    pub active_account: Option<String>,
}
#[derive(Debug, Default, Clone)]
pub struct Explorer {
    pub files: Vec<String>,
}

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

pub enum StateAction {
    AppState(AppState),
    Sources(Sources),
    Accounts(Accounts),
    Explorer(Explorer),
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
            explorer: Explorer { files: vec![] },
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
                        Action::SetSource(source_idx) => {
                            let bucket = self.app_state.sources.available_sources.get(source_idx).map(|val| val.to_string());
                            self.app_state.sources.active_source = bucket;
                            // should do a list for the folders and files in root and handle
                            // unauthenticated errors
                        },
                        Action::SetAccount(account_idx) => {
                            let account = self.app_state.accounts.available_accounts.get(account_idx).map(|val| val.as_str()).unwrap_or("default");
                            self.app_state.accounts.active_account = Some(account.to_string());
                            // should recreate aws client with new selected account
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

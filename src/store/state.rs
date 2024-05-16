use anyhow::Result;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::{action::Action, providers::AwsClient};

#[derive(Debug, Default, Clone)]
pub struct Sources {
    pub available_sources: Vec<String>,
    pub active_idx: Option<usize>,
}
#[derive(Debug, Default, Clone)]
pub struct Accounts {
    pub available_accounts: Vec<String>,
    pub active_idx: Option<usize>,
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
    client: AwsClient,
}

impl State {
    pub async fn new() -> (Self, UnboundedReceiver<AppState>) {
        let (tx, rx) = mpsc::unbounded_channel();
        let client = AwsClient::new().await;
        let accounts = AwsClient::list_accounts();
        let app_state = AppState {
            sources: Sources {
                available_sources: vec![
                    "test1", "test2", "test3", "test1", "test2", "test3", "test1", "test2",
                    "test3", "test1", "test2", "test3", "test1", "test2", "test3", "test1",
                    "test2", "test3", "test1", "test2", "test3", "test1", "test2", "test3",
                ]
                .iter()
                .map(|val| val.to_string())
                .collect(),
                active_idx: None,
            },
            explorer: Explorer { files: vec![] },
            accounts: Accounts {
                active_idx: None,
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
                            self.app_state.sources.active_idx = Some(source_idx);
                            // should do a list for the folders and files in root and handle
                            // unauthenticated errors
                        },
                        Action::SetAccount(account_idx) => {
                            self.app_state.accounts.active_idx = Some(account_idx);
                            // should recreate aws client with new selected account
                        },
                    }
                }
            }
        }
    }
}

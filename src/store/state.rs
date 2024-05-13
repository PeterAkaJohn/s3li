use anyhow::Result;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::action::Action;

#[derive(Debug, Default, Clone)]
struct Sources {
    available_sources: Vec<String>,
    active_idx: Option<usize>,
}
#[derive(Debug, Default, Clone)]
struct Accounts {
    available_accounts: Vec<String>,
    active_idx: Option<usize>,
}

#[derive(Debug, Default, Clone)]
pub struct AppState {
    sources: Sources,
}

pub struct State {
    pub app_state: AppState,
    pub tx: UnboundedSender<AppState>,
}

impl State {
    pub fn new() -> (Self, UnboundedReceiver<AppState>) {
        let (tx, rx) = mpsc::unbounded_channel();
        (
            Self {
                tx,
                app_state: AppState::default(),
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
                            self.app_state.sources.active_idx = Some(account_idx);
                            // should recreate aws client with new selected account
                        },
                    }
                }
            }
        }
    }
}


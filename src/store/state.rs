mod app_state;
mod state_events;
pub mod ui_state;
pub use app_state::{AppState, DashboardComponents};
pub use state_events::StateEvents;

use std::sync::Arc;

use anyhow::Result;
use tokio::sync::{
    mpsc::{self, UnboundedReceiver, UnboundedSender},
    Mutex,
};

use crate::{action::Action, providers::AwsClient};

use super::{
    accounts::Accounts,
    action_manager::ActionManager,
    explorer::Explorer,
    notifications::Notifications,
    sources::{buckets::Buckets, Sources},
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

    pub async fn start(&mut self, mut ui_rx: UnboundedReceiver<Action>) -> Result<()> {
        // we need to send first state to unlock the ui
        self.tx
            .send(StateEvents::UpdateState(self.app_state.clone().into()))?;
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
                        _ => {
                            self.app_state = self.app_state.handle_state_action(action, self.tx.clone()).await?;
                        },
                    };
                }
            }
        }
    }
}

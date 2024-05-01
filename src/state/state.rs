use anyhow::Result;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::action::Action;

#[derive(Debug, Default, Clone)]
pub struct AppState {
    pub counter: u16
}

pub struct State {
    pub app_state: AppState,
    pub tx: UnboundedSender<AppState>
}

impl State {
    pub fn new() -> (Self, UnboundedReceiver<AppState>) {
        let (tx, rx) = mpsc::unbounded_channel();
        (Self {tx, app_state: AppState::default()}, rx)
    }

    pub async fn start(self, mut ui_rx: UnboundedReceiver<Action>) -> Result<()> {
        // we need to send first state to unlock the ui
        self.tx.send(self.app_state.clone())?;
        // need to loop over ui_rx to react to user input
        let result = loop {
            tokio::select! {
                Some(action) = ui_rx.recv() => {
                    // println!("{:?}", action);
                    // println!("received message from ui, trigger action...");
                    match action {
                        Action::Quit => break,
                        Action::Tick => todo!(),
                        Action::Render => todo!(),
                        Action::Key(_) => todo!(),
                    }
                }
            }
        };
        Ok(result)
    }
}
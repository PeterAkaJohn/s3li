use std::sync::Arc;

use anyhow::{Ok, Result};
use providers::AwsClient;
use store::state::State;
use tokio::sync::Mutex;
use tui::ui::Ui;
mod action;
mod logger;
mod providers;
mod store;
mod tui;

#[tokio::main]
async fn main() -> Result<()> {
    let (ui, ui_rx) = Ui::new();
    let client = Arc::new(Mutex::new(AwsClient::new().await));

    let (mut state, state_rx) = State::new(client.clone()).await;

    let _result = tokio::try_join!(ui.start(state_rx), state.start(ui_rx));

    Ok(())
}

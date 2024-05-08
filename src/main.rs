use anyhow::{Ok, Result};
use state::State;
use ui::Ui;
mod action;
mod state;
mod ui;

#[tokio::main]
async fn main() -> Result<()> {
    let (ui, ui_rx) = Ui::new();

    let (state, state_rx) = State::new();

    let result = tokio::try_join!(ui.start(state_rx), state.start(ui_rx));

    Ok(())
}


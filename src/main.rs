use anyhow::{Ok, Result};
use store::state::State;
use tui::ui::Ui;
mod action;
mod store;
mod tui;

#[tokio::main]
async fn main() -> Result<()> {
    let (ui, ui_rx) = Ui::new();

    let (mut state, state_rx) = State::new();

    let _result = tokio::try_join!(ui.start(state_rx), state.start(ui_rx));

    Ok(())
}

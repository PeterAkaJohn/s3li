use std::{
    io::{self, Stdout},
    time::Duration,
};

use anyhow::{Context, Result};
use crossterm::{
    event::{
        DisableMouseCapture, EnableMouseCapture, Event, EventStream, KeyCode, KeyEvent,
        KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::StreamExt;
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::{action::Action, store::state::StateEvents, tui::components::traits::Component};

use super::screens::dashboard::Dashboard;

pub struct Ui {
    pub tx: UnboundedSender<Action>,
}

impl Ui {
    pub fn new() -> (Self, UnboundedReceiver<Action>) {
        let (tx, rx) = mpsc::unbounded_channel();
        (Self { tx }, rx)
    }

    pub async fn start(self, mut state_rx: UnboundedReceiver<StateEvents>) -> Result<()> {
        let mut dash = if let StateEvents::UpdateState(state) = state_rx.recv().await.unwrap() {
            Dashboard::new(&state, self.tx.clone())
        } else {
            panic!("should always have the first state");
        };

        let mut terminal = setup_terminal()?;

        // need to loop and react to state_rx messages and pass it to app
        // need to setup crossterm events and collect them
        let mut term_events = EventStream::new();

        let mut ticker = tokio::time::interval(Duration::from_millis(250));

        let result: Result<()> = loop {
            tokio::select! {
                _ = ticker.tick() => {
                    // println!("ticking");
                },
                maybe_events = term_events.next() => {
                    if let Some(Ok(Event::Key(event))) = maybe_events {
                        if let KeyEvent{
                                code: KeyCode::Char('q'),
                                modifiers: KeyModifiers::CONTROL | KeyModifiers::SHIFT,
                                ..
                            } = event {
                            self.tx.send(Action::Quit)?;
                            break Ok(());
                        }
                        dash.handle_key_events(event);
                    }
                },
                Some(updated_state) = state_rx.recv() => {
                    match updated_state {
                        StateEvents::UpdateState(state) => {
                            dash = dash.refresh_components(&state);
                        }
                        StateEvents::Alert(alert) => {
                            dash.handle_alert(alert);
                        }

                    }
                },
            }

            if let Err(e) = terminal
                .draw(|frame| dash.render(frame, frame.size(), None))
                .context("cannot render")
            {
                println!("error during drawing");
                break Err(e);
            }
        };

        restore_terminal(&mut terminal)?;
        result
    }
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();

    enable_raw_mode()?;

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    Ok(terminal.show_cursor()?)
}

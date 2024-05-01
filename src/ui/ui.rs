use std::{io::{self, Stdout}, time::Duration};

use anyhow::Result;
use crossterm::{event::{DisableMouseCapture, EnableMouseCapture, Event, EventStream}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use futures::StreamExt;
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::{action::Action, state::state::AppState, ui::component::Component};

use super::dashboard::Dashboard;

pub struct Ui {
    pub tx: UnboundedSender<Action>
}

impl Ui {
    pub fn new() -> (Self, UnboundedReceiver<Action>) {
        let (tx, rx) = mpsc::unbounded_channel();
        (Self {tx}, rx)
    }

    pub async fn start(self, mut state_rx: UnboundedReceiver<AppState>) -> Result<()> {

        let mut dash = {
            let state = state_rx.recv().await.unwrap();

            Dashboard::new(&state, self.tx.clone())
        };
        
        let mut terminal = setup_terminal()?; 

        // need to loop and react to state_rx messages and pass it to app
        // need to setup crossterm events and collect them
        let mut term_events = EventStream::new();

        // need also a tick for some reason
        let mut ticker = tokio::time::interval(Duration::from_millis(250));

        let result = loop {
            tokio::select! {
                _ = ticker.tick() => {
                    // println!("ticking");
                    ()
                },
                Some(state) = state_rx.recv() => {
                    // println!("{:?}", state);
                    // println!("received appstate from state, rendering...");
                },
                maybe_events = term_events.next() => {
                    match maybe_events {
                        Some(Ok(Event::Key(event))) => {
                            // println!("received event from terminal, sending to ui, possible rerendering {:?}", event);
                            if event.code == crossterm::event::KeyCode::Char('q') {
                                self.tx.send(Action::Quit)?;
                                break;
                            }
                            dash.handle_key_events(event);
                        }
                        _ => ()
                    }
                }
            }

            if let Err(e) = terminal.draw(|frame| dash.render(frame, frame.size(),())) {
                println!("error during drawing");
                return Err(e.into())
            }
        };

        restore_terminal(&mut terminal)?;
        Ok(result)
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
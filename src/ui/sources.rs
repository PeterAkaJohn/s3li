use crossterm::event::KeyEventKind;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        block::Title, Block, Borders, List, ListItem, ListState, Padding, StatefulWidget, Widget,
    },
};
use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;

use super::component::Component;

#[derive(Debug)]
pub struct Sources<'a> {
    selected_source: Option<&'a str>,
    search_input: Option<String>,
    list_state: ListState,
    available_sources: Vec<&'a str>,
    ui_tx: UnboundedSender<Action>,
}

impl<'a> Sources<'a> {
    pub fn new(available_sources: Vec<&'a str>, ui_tx: UnboundedSender<Action>) -> Self {
        Self {
            available_sources,
            selected_source: None,
            search_input: None,
            ui_tx,
            list_state: ListState::default(),
        }
    }

    fn unselect(&mut self) {
        self.list_state.select(None);
    }
    fn select_next(&mut self) {
        let idx = match self.list_state.selected() {
            Some(selected_idx) => {
                if selected_idx == self.available_sources.len() - 1 {
                    0                    
                } else {
                    selected_idx + 1
                }
            },
            None => 0,
        };
        self.list_state.select(Some(idx))
    }
    fn select_previous(&mut self) {
        let idx = match self.list_state.selected() {
            Some(selected_idx) => {
                if selected_idx == 0 {
                    self.available_sources.len() - 1
                } else {
                    selected_idx - 1
                }
            },
            None => self.available_sources.len() - 1,
        };
        self.list_state.select(Some(idx))
    }
}

impl Component for Sources<'_> {
    fn render(&mut self, f: &mut ratatui::prelude::Frame, area: ratatui::prelude::Rect, props: ()) {
        let sources = Block::default()
            .borders(Borders::ALL)
            .padding(Padding::horizontal(1))
            .title(Title::default().content("Sources"));

        let active_style = Style::default().fg(Color::LightGreen);
        let default_style = Style::default().fg(Color::White);
        let list_items = self
            .available_sources
            .iter()
            .map(|key| {
                ListItem::new(Line::from(Span::styled(
                    format!("{: <25}", key),
                    default_style,
                )))
            })
            .collect::<Vec<ListItem>>();

        let list = List::new(list_items)
            .block(sources)
            .highlight_symbol(">")
            .scroll_padding(2)
            .highlight_style(active_style)
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

        f.render_stateful_widget(list, area, &mut self.list_state);
    }

    fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        if self.available_sources.len() <= 0 {
            return;
        }
        match key.code {
            crossterm::event::KeyCode::Esc => {
                self.unselect();
            }
            crossterm::event::KeyCode::Up => {
                self.select_previous();
            }
            crossterm::event::KeyCode::Down => {
                self.select_next();
            }
            _ => {}
        };
    }
}

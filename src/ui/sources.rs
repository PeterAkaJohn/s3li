use crossterm::{event::KeyEventKind, style::style};
use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{
        block::Title, Block, Borders, List, ListItem, ListState, Padding, StatefulWidget, Widget,
    },
};
use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;

use super::{
    component::{get_block_container, Component, ComponentProps},
    list::WithList,
};

#[derive(Debug)]
pub struct Sources<'a> {
    selected_source: Option<&'a str>,
    search_input: Option<String>,
    list_state: ListState,
    available_sources: Vec<&'a str>,
    ui_tx: UnboundedSender<Action>,
}

impl WithList for Sources<'_> {
    fn get_list_items_len(&self) -> usize {
        self.available_sources.len()
    }

    fn get_list_state_selected(&self) -> Option<usize> {
        self.list_state.selected()
    }

    fn set_selected(&mut self, idx: Option<usize>) {
        self.list_state.select(idx);
    }
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
}

impl Component for Sources<'_> {
    fn render(
        &mut self,
        f: &mut ratatui::prelude::Frame,
        area: ratatui::prelude::Rect,
        props: Option<ComponentProps>,
    ) {
        let sources = get_block_container("Sources", props);
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
        if self.available_sources.is_empty() {
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

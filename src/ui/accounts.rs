use crossterm::event::KeyEventKind;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{block::Title, Block, Borders, List, ListItem, ListState, Padding},
};
use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;

use super::{
    component::{get_block_container, Component, ComponentProps},
    list::WithList,
};

#[derive(Debug)]
pub struct Accounts<'a> {
    selected_account: Option<&'a str>,
    list_state: ListState,
    available_accounts: Vec<&'a str>,
    ui_tx: UnboundedSender<Action>,
}

impl WithList for Accounts<'_> {
    fn get_list_items_len(&self) -> usize {
        self.available_accounts.len()
    }

    fn get_list_state_selected(&self) -> Option<usize> {
        self.list_state.selected()
    }

    fn set_selected(&mut self, idx: Option<usize>) {
        self.list_state.select(idx);
    }
}

impl<'a> Accounts<'a> {
    pub fn new(available_sources: Vec<&'a str>, ui_tx: UnboundedSender<Action>) -> Self {
        Self {
            available_accounts: available_sources,
            selected_account: None,
            ui_tx,
            list_state: ListState::default(),
        }
    }
}

impl Component for Accounts<'_> {
    fn render(
        &mut self,
        f: &mut ratatui::prelude::Frame,
        area: ratatui::prelude::Rect,
        props: Option<ComponentProps>,
    ) {
        let accounts = get_block_container("Accounts", props);

        let active_style = Style::default().fg(Color::LightGreen);
        let default_style = Style::default().fg(Color::White);
        let list_items = self
            .available_accounts
            .iter()
            .map(|key| {
                ListItem::new(Line::from(Span::styled(
                    format!("{: <25}", key),
                    default_style,
                )))
            })
            .collect::<Vec<ListItem>>();

        let list = List::new(list_items)
            .block(accounts)
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
        if self.available_accounts.is_empty() {
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

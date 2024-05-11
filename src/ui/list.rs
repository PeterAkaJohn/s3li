use crossterm::event::KeyEventKind;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{
        block::Title, Block, Borders, List, ListItem, ListState, Padding, Paragraph, Scrollbar,
        ScrollbarOrientation, ScrollbarState,
    },
};

use super::component::{Component, ComponentProps, WithContainer};

pub trait WithList {
    fn get_list_items_len(&self) -> usize;
    fn get_list_state_selected(&self) -> Option<usize>;
    fn set_selected(&mut self, idx: Option<usize>);
    fn unselect(&mut self) {
        self.set_selected(None);
    }
    fn select_next(&mut self) {
        let list_state_selected = self.get_list_state_selected();
        let list_len = self.get_list_items_len();
        let idx = match list_state_selected {
            Some(selected_idx) => {
                if selected_idx == list_len - 1 {
                    0
                } else {
                    selected_idx + 1
                }
            }
            None => 0,
        };
        self.set_selected(Some(idx))
    }
    fn select_previous(&mut self) {
        let list_state_selected = self.get_list_state_selected();
        let list_len = self.get_list_items_len();
        let idx = match list_state_selected {
            Some(selected_idx) => {
                if selected_idx == 0 {
                    list_len - 1
                } else {
                    selected_idx - 1
                }
            }
            None => list_len - 1,
        };
        self.set_selected(Some(idx))
    }
}

pub struct ListComponent<'a, T> {
    list_state: ListState,
    items: Vec<T>,
    title: &'a str,
}

impl<'a> ListComponent<'a, &str> {
    pub fn new(title: &'a str, items: Vec<&'a str>) -> ListComponent<'a, &'a str> {
        ListComponent {
            list_state: ListState::default(),
            items,
            title,
        }
    }
}

impl WithList for ListComponent<'_, &str> {
    fn get_list_items_len(&self) -> usize {
        self.items.len()
    }

    fn get_list_state_selected(&self) -> Option<usize> {
        self.list_state.selected()
    }

    fn set_selected(&mut self, idx: Option<usize>) {
        self.list_state.select(idx);
    }
}

impl WithContainer<'_> for ListComponent<'_, &str> {}

impl Component for ListComponent<'_, &str> {
    fn render(
        &mut self,
        f: &mut ratatui::prelude::Frame,
        area: ratatui::prelude::Rect,
        props: Option<ComponentProps>,
    ) {
        let container = self.with_container(self.title, &props);
        if let Some(ComponentProps { selected: false }) = props {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage((100 - 20) / 2),
                    Constraint::Percentage(20),
                    Constraint::Percentage((100 - 20) / 2),
                ])
                .split(container.inner(area));

            let content = Paragraph::new(self.title).alignment(ratatui::layout::Alignment::Center);
            f.render_widget(container, area);
            f.render_widget(content, layout[1]);
        } else {
            let active_style = Style::default().fg(Color::Green).bg(Color::LightBlue);
            let default_style = Style::default().fg(Color::White);
            let list_items = self
                .items
                .iter()
                .map(|key| {
                    ListItem::new(Line::from(Span::styled(
                        format!("{: <25}", key),
                        default_style,
                    )))
                })
                .collect::<Vec<ListItem>>();

            let list = List::new(list_items)
                .block(container)
                .scroll_padding(2)
                .highlight_style(active_style)
                .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

            f.render_stateful_widget(list, area, &mut self.list_state);
        }
    }

    fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        if self.items.is_empty() {
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

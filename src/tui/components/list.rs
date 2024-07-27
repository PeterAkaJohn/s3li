use std::usize;

use crossterm::event::KeyEventKind;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{List, ListItem, ListState, Paragraph},
};

use crate::tui::components::functions::add_white_space_till_width_if_needed;

use super::traits::{
    Component, ComponentProps, SelectionDirection, WithContainer, WithList, WithSelection,
};

#[derive(Debug)]
pub enum ListMode {
    Normal,
    Selection,
}

pub struct ListComponent<T> {
    list_state: ListState,
    items: Vec<T>,
    title: String,
    active_idx: Option<usize>,
    selection: Option<(usize, usize)>,
    mode: ListMode,
}

impl ListComponent<String> {
    pub fn new(
        title: String,
        items: Vec<String>,
        active_element: Option<String>,
    ) -> ListComponent<String> {
        let active_idx = active_element.and_then(|acc| items.iter().position(|val| *val == acc));
        let selected_idx = if !items.is_empty() { Some(0) } else { None };
        ListComponent {
            list_state: ListState::default().with_selected(if active_idx.is_some() {
                active_idx
            } else {
                selected_idx
            }),
            items,
            title,
            active_idx,
            selection: None,
            mode: ListMode::Normal,
        }
    }
    pub fn set_active_idx(&mut self, active_idx: Option<usize>) {
        self.active_idx = active_idx;
    }

    pub fn get_active_idx(&self) -> Option<usize> {
        self.active_idx
    }

    pub fn get_selected_item_value(&self) -> &str {
        self.items
            .get(self.list_state.selected().expect("should always be valid"))
            .expect("should always exist")
    }
}

impl WithList for ListComponent<String> {
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

impl WithContainer<'_> for ListComponent<String> {}

impl WithSelection for ListComponent<String> {
    fn start_selection(&mut self, idx: usize) {
        self.mode = ListMode::Selection;
        self.selection = Some((idx, idx));
    }
    fn end_selection(&mut self) {
        self.mode = ListMode::Normal;
        self.selection = None;
    }

    fn get_selection(&self) -> &Option<(usize, usize)> {
        &self.selection
    }

    fn resize_selection(&mut self, direction: SelectionDirection) {
        if matches!(direction, SelectionDirection::Up) {
            self.select_previous();
        } else {
            self.select_next();
        };
        let idx = self.get_list_state_selected();
        if let (Some(idx), Some((min, max))) = (idx, self.selection) {
            self.selection = self.compute_selection(min, max, idx, direction);
        }
    }
}

impl Component for ListComponent<String> {
    fn render(
        &mut self,
        f: &mut ratatui::prelude::Frame,
        area: ratatui::prelude::Rect,
        props: Option<ComponentProps>,
    ) {
        let selected_item = if let Some(selected_idx) = self.active_idx {
            self.items.get(selected_idx)
        } else {
            None
        };
        let container = self.with_container(&self.title, &props);
        if let Some(ComponentProps { selected: false }) = props {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage((100 - 20) / 2),
                    Constraint::Percentage(20),
                    Constraint::Percentage((100 - 20) / 2),
                ])
                .split(container.inner(area));

            let title = if let Some(item) = selected_item {
                format!("{} ({})", self.title, item)
            } else {
                format!("{} ({})", self.title, &self.items.len())
            };

            let content = Paragraph::new(title).alignment(ratatui::layout::Alignment::Center);
            f.render_widget(container, area);
            f.render_widget(content, layout[1]);
        } else {
            let active_style = Style::default().fg(Color::Green).bg(Color::LightBlue);
            let default_style = Style::default().fg(Color::White);
            let list_items = self
                .items
                .iter()
                .enumerate()
                .map(|(index, key)| {
                    let is_selected = if let Some((min_bound, max_bound)) = self.selection {
                        index <= max_bound && index >= min_bound
                    } else {
                        false
                    };
                    let line_item_label = add_white_space_till_width_if_needed(
                        &format!("{: <25}", key),
                        area.width as usize,
                    );
                    ListItem::new(Line::from(Span::styled(
                        line_item_label,
                        if is_selected {
                            active_style
                        } else {
                            default_style
                        },
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
            crossterm::event::KeyCode::Esc if matches!(self.mode, ListMode::Selection) => {
                self.end_selection();
            }
            crossterm::event::KeyCode::Char('v') => {
                if matches!(self.mode, ListMode::Normal) {
                    let current_idx = self.get_list_state_selected();
                    if let Some(idx) = current_idx {
                        self.start_selection(idx);
                    }
                } else {
                    self.end_selection();
                }
            }
            crossterm::event::KeyCode::Esc => {
                self.unselect();
                self.set_active_idx(None);
            }
            crossterm::event::KeyCode::Up | crossterm::event::KeyCode::Char('k') => {
                if matches!(self.mode, ListMode::Selection) {
                    self.resize_selection(SelectionDirection::Up);
                } else {
                    self.select_previous();
                }
            }
            crossterm::event::KeyCode::Down | crossterm::event::KeyCode::Char('j') => {
                if matches!(self.mode, ListMode::Selection) {
                    self.resize_selection(SelectionDirection::Down);
                } else {
                    self.select_next();
                }
            }
            crossterm::event::KeyCode::Enter => {
                self.set_active_idx(self.get_list_state_selected());
            }
            _ => {}
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::tui::components::traits::{SelectionDirection, WithSelection};

    use super::ListComponent;

    #[test]
    fn with_selection_list_component() {
        let items: Vec<String> = ["test1", "test2", "test3", "test4", "test5", "test6"]
            .iter()
            .map(|item| item.to_string())
            .collect();
        let active_element = None;
        let mut list_component = ListComponent::new("test".to_string(), items, active_element);

        assert!(matches!(
            list_component.mode,
            crate::tui::components::list::ListMode::Normal
        ));
        list_component.start_selection(0);
        assert!(matches!(
            list_component.mode,
            crate::tui::components::list::ListMode::Selection
        ));
        assert_eq!(list_component.selection, Some((0, 0)));

        list_component.resize_selection(SelectionDirection::Down);
        assert_eq!(list_component.selection, Some((0, 1)));
        list_component.resize_selection(SelectionDirection::Up);
        assert_eq!(list_component.selection, Some((0, 0)));
    }
}

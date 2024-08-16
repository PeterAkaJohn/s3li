use std::usize;

use crossterm::event::{KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{List, ListItem, ListState, Paragraph},
};

use crate::{
    logger::LOGGER,
    tui::{
        components::functions::add_white_space_till_width_if_needed,
        key_event::{ExecuteEventListener, S3liEventListener, S3liKeyEvent},
    },
};

use super::traits::{
    Component, ComponentProps, Selection, SelectionDirection, WithBlockSelection, WithContainer,
    WithList, WithMultiSelection,
};

#[derive(Debug)]
pub enum ListMode {
    Normal,
    Selection,
    Multi,
}

pub struct ListComponent<T> {
    list_state: ListState,
    items: Vec<T>,
    title: String,
    active_idx: Option<usize>,
    selection: Vec<usize>,
    mode: ListMode,
    listeners: Vec<S3liEventListener<Self>>,
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
            selection: vec![],
            mode: ListMode::Normal,
            listeners: Self::register_listeners(),
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

    fn register_listeners() -> Vec<S3liEventListener<Self>> {
        vec![
            (
                S3liKeyEvent::new(crossterm::event::KeyCode::Char(' '), KeyModifiers::NONE),
                Self::select_multi,
            ),
            (
                S3liKeyEvent::new(crossterm::event::KeyCode::Esc, KeyModifiers::NONE),
                Self::cancel,
            ),
            (
                S3liKeyEvent::new(crossterm::event::KeyCode::Char('v'), KeyModifiers::NONE),
                Self::visual_block,
            ),
            (
                S3liKeyEvent::new(crossterm::event::KeyCode::Char('k'), KeyModifiers::NONE),
                Self::move_up,
            ),
            (
                S3liKeyEvent::new(crossterm::event::KeyCode::Char('j'), KeyModifiers::NONE),
                Self::move_down,
            ),
            (
                S3liKeyEvent::new(crossterm::event::KeyCode::Enter, KeyModifiers::NONE),
                Self::confirm_selection,
            ),
        ]
    }

    fn select_multi(&mut self) {
        self.mode = ListMode::Multi;
        let current_idx = self.get_list_state_selected();
        if let Some(idx) = current_idx {
            self.toggle_selection(idx);
        }
        if self.selection.is_empty() {
            self.mode = ListMode::Normal;
        }
        LOGGER.info(&format!("selection: {:?}", self.selection));
    }

    fn cancel(&mut self) {
        if matches!(self.mode, ListMode::Selection | ListMode::Multi) {
            self.end_selection();
        } else {
            self.unselect();
            self.set_active_idx(None);
        }
    }

    fn visual_block(&mut self) {
        if matches!(self.mode, ListMode::Normal) {
            let current_idx = self.get_list_state_selected();
            if let Some(idx) = current_idx {
                self.start_selection(idx);
            }
        } else {
            self.end_selection();
        }
    }

    fn move_up(&mut self) {
        if matches!(self.mode, ListMode::Selection) {
            self.resize_selection(SelectionDirection::Up);
        } else {
            self.select_previous();
        }
    }

    fn move_down(&mut self) {
        if matches!(self.mode, ListMode::Selection) {
            self.resize_selection(SelectionDirection::Down);
        } else {
            self.select_next();
        }
    }

    fn confirm_selection(&mut self) {
        self.set_active_idx(self.get_list_state_selected());
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

impl WithBlockSelection for ListComponent<String> {
    fn start_selection(&mut self, idx: usize) {
        self.mode = ListMode::Selection;
        self.selection = vec![idx];
    }
    fn end_selection(&mut self) {
        self.mode = ListMode::Normal;
        self.selection = vec![];
    }
    fn get_selection(&self) -> Option<Selection> {
        let selection = self.selection.iter().min().zip(self.selection.iter().max());

        selection.map(|val| (*val.0, *val.1))
    }
    fn set_selection(&mut self, selection: Option<Selection>) {
        if let Some((min, max)) = selection {
            let range: Vec<usize> = (min..=max).collect();
            self.selection = range;
        }
    }
}

impl WithMultiSelection for ListComponent<String> {
    fn get_multi_selection(&self) -> &Vec<usize> {
        &self.selection
    }

    fn set_multi_selection(&mut self, selection: Vec<usize>) {
        self.selection = selection;
    }
}

impl ExecuteEventListener for ListComponent<String> {
    fn get_event_listeners(&self) -> &Vec<S3liEventListener<Self>> {
        &self.listeners
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
                    let is_selected = self.selection.contains(&index);
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
        self.execute(key)
    }
}

#[cfg(test)]
mod tests {
    use crate::tui::components::traits::{SelectionDirection, WithBlockSelection};

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
        assert_eq!(list_component.selection, vec![0]);

        list_component.resize_selection(SelectionDirection::Down);
        assert_eq!(list_component.selection, vec![0, 1]);
        list_component.resize_selection(SelectionDirection::Up);
        assert_eq!(list_component.selection, vec![0]);
        list_component.resize_selection(SelectionDirection::Down);
        list_component.resize_selection(SelectionDirection::Down);
        list_component.resize_selection(SelectionDirection::Down);
        assert_eq!(list_component.selection, vec![0, 1, 2, 3]);
    }
}

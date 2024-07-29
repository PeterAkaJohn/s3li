use anyhow::Result;
use crossterm::event::KeyEvent;
use ratatui::{
    layout::{self, Rect},
    style::{self, Stylize},
    widgets::{self, Block},
    Frame,
};

#[derive(Clone)]
pub struct ComponentProps {
    pub selected: bool,
}

pub trait Component {
    fn init(&mut self) -> Result<()> {
        Ok(())
    }
    //   fn handle_events(&mut self, event: Option<Event>) -> Action {
    //     match event {
    //       Some(Event::Quit) => Action::Quit,
    //       Some(Event::Tick) => Action::Tick,
    //       Some(Event::Key(key_event)) => self.handle_key_events(key_event),
    //       Some(Event::Mouse(mouse_event)) => self.handle_mouse_events(mouse_event),
    //       Some(Event::Resize(x, y)) => Action::Resize(x, y),
    //       Some(_) => Action::Noop,
    //       None => Action::Noop,
    //     }
    //   }
    fn handle_key_events(&mut self, key: KeyEvent);
    //   fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Action {
    //     Action::Noop
    //   }
    //   fn update(&mut self, action: Action) -> Action {
    //     Action::Noop
    //   }
    fn render(&mut self, f: &mut Frame, area: Rect, props: Option<ComponentProps>);
}

pub trait WithContainer<'a> {
    fn with_container(&self, container_title: &'a str, props: &Option<ComponentProps>) -> Block<'a>
    where
        Self: Sized,
    {
        let container = Block::default()
            .borders(widgets::Borders::ALL)
            .border_type(widgets::BorderType::Rounded)
            .padding(widgets::block::Padding::horizontal(1));
        match props {
            Some(ComponentProps { selected: true }) => container
                .border_style(style::Style::default().green())
                .title_alignment(layout::Alignment::Center)
                .title(widgets::block::Title::default().content(container_title)),
            _ => container.border_style(style::Style::default()),
        }
    }
}

pub trait WithList {
    fn get_list_items_len(&self) -> usize;
    fn get_list_state_selected(&self) -> Option<usize>;
    fn set_selected(&mut self, idx: Option<usize>);
    fn unselect(&mut self) {
        self.set_selected(None);
    }
    fn get_next(&self) -> Option<usize> {
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
        Some(idx)
    }
    fn select_next(&mut self) {
        let next_idx = self.get_next();
        self.set_selected(next_idx);
    }
    fn get_previous(&self) -> Option<usize> {
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
        Some(idx)
    }
    fn select_previous(&mut self) {
        let previous_idx = self.get_previous();
        self.set_selected(previous_idx);
    }
}

pub enum SelectionDirection {
    Up,
    Down,
}

pub type Selection = (usize, usize);
pub trait WithBlockSelection: WithList {
    fn start_selection(&mut self, idx: usize);
    fn end_selection(&mut self);
    fn get_selection(&self) -> Option<Selection>;
    fn set_selection(&mut self, selection: Option<Selection>);
    fn compute_selection(
        &self,
        min: usize,
        max: usize,
        idx: usize,
        direction: SelectionDirection,
    ) -> Option<Selection> {
        match direction {
            SelectionDirection::Up if idx > min && max > 0 && idx == max - 1 => Some((min, idx)),
            SelectionDirection::Up if min == max && idx > max => Some((min, idx)),
            SelectionDirection::Up if idx < min => Some((idx, max)),
            SelectionDirection::Down if idx < max && idx == min + 1 => Some((idx, max)),
            SelectionDirection::Down if min == max && idx < min => Some((idx, max)),
            SelectionDirection::Down if idx > max => Some((min, idx)),
            _ => Some((idx, idx)),
        }
    }
    fn resize_selection(&mut self, direction: SelectionDirection) {
        if matches!(direction, SelectionDirection::Up) {
            self.select_previous();
        } else {
            self.select_next();
        };
        let idx = self.get_list_state_selected();
        if let (Some(idx), Some((min, max))) = (idx, self.get_selection()) {
            self.set_selection(self.compute_selection(min, max, idx, direction));
        }
    }
}

pub trait WithMultiSelection: WithBlockSelection {
    fn toggle_selection(&mut self, idx: usize) {
        let mut current_selection = self.get_multi_selection();

        if let Some(existing_position) = current_selection.iter().position(|val| *val == idx) {
            current_selection.remove(existing_position);
        } else {
            current_selection.push(idx);
        }

        current_selection.sort();
        self.set_multi_selection(current_selection.to_vec());
    }
    fn get_multi_selection(&self) -> Vec<usize>;
    fn set_multi_selection(&mut self, selection: Vec<usize>);
}

#[cfg(test)]
mod tests {
    use super::{Selection, WithBlockSelection, WithList, WithMultiSelection};

    #[derive(Default)]
    struct MockList {
        selection: Option<Selection>,
        multi_selection: Vec<usize>,
        pub next_idx: usize,
    }

    impl WithList for MockList {
        fn get_list_items_len(&self) -> usize {
            0
        }

        fn get_list_state_selected(&self) -> Option<usize> {
            Some(0)
        }

        fn set_selected(&mut self, _idx: Option<usize>) {}
    }

    impl WithBlockSelection for MockList {
        fn start_selection(&mut self, idx: usize) {
            self.selection = Some((idx, idx));
        }

        fn end_selection(&mut self) {
            self.selection = None;
        }

        fn set_selection(&mut self, selection: Option<Selection>) {
            self.selection = selection;
        }

        fn get_selection(&self) -> Option<Selection> {
            self.selection
        }

        fn resize_selection(&mut self, direction: super::SelectionDirection) {
            let idx = self.next_idx;
            let (min, max) = self.selection.expect("should not fail within test");
            self.selection = self.compute_selection(min, max, idx, direction)
        }
    }

    impl WithMultiSelection for MockList {
        fn get_multi_selection(&self) -> Vec<usize> {
            self.multi_selection.clone()
        }

        fn set_multi_selection(&mut self, selection: Vec<usize>) {
            self.multi_selection = selection
        }
    }

    #[test]
    fn test_with_multi_selection() {
        let mut mocklist = MockList::default();
        mocklist.toggle_selection(0);
        assert_eq!(mocklist.multi_selection, vec![0]);
        mocklist.toggle_selection(20);
        assert_eq!(mocklist.multi_selection, vec![0, 20]);
        mocklist.toggle_selection(10);
        assert_eq!(mocklist.multi_selection, vec![0, 10, 20]);
        mocklist.toggle_selection(10);
        assert_eq!(mocklist.multi_selection, vec![0, 20]);
        mocklist.toggle_selection(20);
        assert_eq!(mocklist.multi_selection, vec![0]);
        mocklist.toggle_selection(0);
        assert_eq!(mocklist.multi_selection, vec![]);
    }

    #[test]
    fn test_with_selection_trait_compute_selection() {
        let mut mocklist = MockList::default();
        mocklist.start_selection(0);
        assert_eq!(mocklist.selection, Some((0, 0)));

        mocklist.next_idx = 1;
        mocklist.resize_selection(super::SelectionDirection::Down);
        assert_eq!(mocklist.selection, Some((0, 1)));

        mocklist.next_idx = 0;
        mocklist.resize_selection(super::SelectionDirection::Up);
        assert_eq!(mocklist.selection, Some((0, 0)));

        mocklist.next_idx = 2;
        mocklist.resize_selection(super::SelectionDirection::Down);
        assert_eq!(mocklist.selection, Some((0, 2)));

        mocklist.end_selection();
        assert_eq!(mocklist.selection, None);

        mocklist.start_selection(4);
        assert_eq!(mocklist.selection, Some((4, 4)));

        mocklist.next_idx = 3;
        mocklist.resize_selection(crate::tui::components::traits::SelectionDirection::Up);
        assert_eq!(mocklist.selection, Some((3, 4)));

        mocklist.next_idx = 2;
        mocklist.resize_selection(crate::tui::components::traits::SelectionDirection::Up);
        assert_eq!(mocklist.selection, Some((2, 4)));

        mocklist.next_idx = 3;
        mocklist.resize_selection(crate::tui::components::traits::SelectionDirection::Down);
        assert_eq!(mocklist.selection, Some((3, 4)));

        mocklist.next_idx = 4;
        mocklist.resize_selection(crate::tui::components::traits::SelectionDirection::Down);
        assert_eq!(mocklist.selection, Some((4, 4)));

        mocklist.next_idx = 5;
        mocklist.resize_selection(crate::tui::components::traits::SelectionDirection::Down);
        assert_eq!(mocklist.selection, Some((4, 5)));

        mocklist.next_idx = 6;
        mocklist.resize_selection(crate::tui::components::traits::SelectionDirection::Down);
        assert_eq!(mocklist.selection, Some((4, 6)));
    }
}

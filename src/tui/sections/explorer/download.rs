use std::ops::Add;

use crossterm::event::KeyModifiers;
use ratatui::{
    layout::{Constraint, Direction, Layout, Margin},
    widgets::{block::Title, Clear},
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action,
    store::explorer::TreeItem,
    tui::{
        components::{
            input::Input,
            popup::WithPopup,
            traits::{Component, ComponentProps, WithContainer},
        },
        key_event::{EventListeners, ExecuteEventListener, S3liKeyEvent, S3liOnChangeEvent},
    },
};

#[derive(Debug)]
pub struct Download {
    pub open: bool,
    pub items: Vec<TreeItem>,
    ui_tx: UnboundedSender<Action>,
    current_file_idx: usize,
    listeners: Vec<EventListeners<Self>>,
}

impl Download {
    pub fn new(ui_tx: UnboundedSender<Action>) -> Download {
        Download {
            ui_tx,
            open: false,
            items: vec![],
            current_file_idx: 0,
            listeners: Self::register_listeners(),
        }
    }

    pub fn init(&mut self, tree_items: Vec<TreeItem>) {
        self.items = tree_items;
        self.open = true;
    }

    fn register_listeners() -> Vec<EventListeners<Self>> {
        vec![
            EventListeners::KeyEvent((
                S3liKeyEvent::new(vec![(crossterm::event::KeyCode::Esc, KeyModifiers::NONE)]),
                Self::exit_component,
            )),
            EventListeners::KeyEvent((
                S3liKeyEvent::new(vec![(crossterm::event::KeyCode::Enter, KeyModifiers::NONE)]),
                Self::confirm,
            )),
            EventListeners::KeyEvent((
                S3liKeyEvent::new(vec![(crossterm::event::KeyCode::Tab, KeyModifiers::NONE)]),
                Self::cycle_current_file,
            )),
            EventListeners::OnChangeEvent((
                S3liOnChangeEvent::new(),
                Self::add_char,
                Self::delete_char,
            )),
        ]
    }

    fn exit_component(&mut self) {
        self.open = false;
    }
    fn confirm(&mut self) {
        // send region to state with ui_tx
        let _ = self.ui_tx.send(Action::Download(self.items.clone()));
        self.open = false;
    }
    fn cycle_current_file(&mut self) {
        self.current_file_idx = if self.current_file_idx == self.items.len() - 1 {
            0
        } else {
            self.current_file_idx.add(1)
        };
    }
    fn delete_char(&mut self) {
        if let Some(item) = self.items.get_mut(self.current_file_idx) {
            item.pop_name_char();
        }
    }
    fn add_char(&mut self, value: char) {
        if let Some(item) = self.items.get_mut(self.current_file_idx) {
            item.push_name_char(value);
        }
    }
}

impl WithPopup for Download {
    fn set_popup_state(&mut self, open: bool) {
        self.open = open;
    }

    fn get_popup_state(&self) -> bool {
        self.open
    }
}

impl WithContainer<'_> for Download {}

impl ExecuteEventListener for Download {
    fn get_event_listeners(&self) -> &Vec<EventListeners<Self>> {
        &self.listeners
    }
}

impl Component for Download {
    fn render(
        &mut self,
        f: &mut ratatui::prelude::Frame,
        _area: ratatui::prelude::Rect,
        props: Option<ComponentProps>,
    ) {
        let mut container = self.with_container("Download file", &props);
        container = container.title(
            Title::from(format!(
                "{} of {}",
                self.current_file_idx + 1,
                self.items.len()
            ))
            .position(ratatui::widgets::block::Position::Bottom)
            .alignment(ratatui::layout::Alignment::Right),
        );

        let current_item = self.items.get(self.current_file_idx);
        if let Some(item) = current_item {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Fill(1), Constraint::Max(3), Constraint::Fill(1)])
                .split(f.size());
            let center_section = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Fill(2),
                    Constraint::Fill(2),
                    Constraint::Fill(2),
                ])
                .split(layout[1])[1];

            let input = Input::new(item.name().to_string(), true);
            f.render_widget(Clear, center_section);
            f.render_widget(container, center_section);
            f.render_widget(input, center_section.inner(&Margin::new(1, 1)));
        }
    }
    fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) {
        self.execute(key)
    }
}

use crossterm::event::KeyModifiers;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::Clear,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action,
    tui::{
        components::{
            input::InputBlock,
            popup::WithPopup,
            traits::{Component, ComponentProps, WithContainer},
        },
        key_event::{EventListeners, ExecuteEventListener, S3liKeyEvent, S3liOnChangeEvent},
    },
};

pub struct Region {
    pub open: bool,
    pub region: String,
    pub new_region: String,
    ui_tx: UnboundedSender<Action>,
    listeners: Vec<EventListeners<Self>>,
}

impl Region {
    pub fn new(region: String, ui_tx: UnboundedSender<Action>) -> Region {
        Region {
            ui_tx,
            open: false,
            region: region.clone(),
            new_region: region.clone(),
            listeners: Self::register_listeners(),
        }
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
            EventListeners::OnChangeEvent((
                S3liOnChangeEvent::new(),
                Self::add_char,
                Self::delete_char,
            )),
        ]
    }

    fn exit_component(&mut self) {
        self.open = false;
        if !self.new_region.eq(&self.region) {
            self.new_region.clone_from(&self.region);
        }
    }
    fn confirm(&mut self) {
        // send region to state with ui_tx
        let _ = self
            .ui_tx
            .send(Action::ChangeRegion(self.new_region.clone()));
        self.open = false;
    }
    fn delete_char(&mut self) {
        self.new_region.pop();
    }
    fn add_char(&mut self, value: char) {
        self.new_region.push(value);
    }
}

impl WithPopup for Region {
    fn set_popup_state(&mut self, open: bool) {
        self.open = open;
    }

    fn get_popup_state(&self) -> bool {
        self.open
    }
}

impl WithContainer<'_> for Region {}

impl ExecuteEventListener for Region {
    fn get_event_listeners(&self) -> &Vec<EventListeners<Self>> {
        &self.listeners
    }
}

impl Component for Region {
    fn render(
        &mut self,
        f: &mut ratatui::prelude::Frame,
        _area: ratatui::prelude::Rect,
        _: Option<ComponentProps>,
    ) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Max(3), Constraint::Fill(1)])
            .split(f.size());
        let center_section = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(2),
                Constraint::Fill(1),
                Constraint::Fill(2),
            ])
            .split(layout[1])[1];

        let input = InputBlock::new(self.new_region.to_string(), "Region".to_string(), true);
        f.render_widget(Clear, center_section);
        f.render_widget(input, center_section);
    }
    fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) {
        self.execute(key);
    }
}

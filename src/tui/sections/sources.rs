use crossterm::event::KeyModifiers;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action,
    tui::{
        components::{
            list::ListComponent,
            traits::{Component, ComponentProps},
        },
        key_event::{ExecuteEventListener, S3liEventListener, S3liKeyEvent},
    },
};

pub struct Sources {
    component: ListComponent<String>,
    ui_tx: UnboundedSender<Action>,
    listeners: Vec<S3liEventListener<Self>>,
}

impl Sources {
    pub fn new(
        items: &Vec<String>,
        active_source: &Option<String>,
        ui_tx: UnboundedSender<Action>,
    ) -> Sources {
        Sources {
            component: ListComponent::new(
                "Sources".to_string(),
                items.to_owned(),
                active_source.to_owned(),
            ),
            ui_tx,
            listeners: Self::register_listeners(),
        }
    }

    fn register_listeners() -> Vec<S3liEventListener<Self>> {
        vec![(
            S3liKeyEvent::new(crossterm::event::KeyCode::Enter, KeyModifiers::NONE),
            Self::enter_pressed,
        )]
    }

    fn enter_pressed(&mut self) {
        if let Some(idx) = self.component.get_active_idx() {
            let _ = self.ui_tx.send(Action::SetSource(idx));
        }
    }
}

impl ExecuteEventListener for Sources {
    fn get_event_listeners(&self) -> &Vec<S3liEventListener<Self>> {
        &self.listeners
    }
}

impl Component for Sources {
    fn render(
        &mut self,
        f: &mut ratatui::prelude::Frame,
        area: ratatui::prelude::Rect,
        props: Option<ComponentProps>,
    ) {
        self.component.render(f, area, props)
    }
    fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) {
        self.component.handle_key_events(key);
        self.execute(key)
    }
}

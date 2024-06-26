use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;

use super::{
    component::{Component, ComponentProps},
    list::ListComponent,
};

pub struct Sources {
    component: ListComponent<String>,
    ui_tx: UnboundedSender<Action>,
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
        }
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
        if let crossterm::event::KeyCode::Enter = key.code {
            if let Some(idx) = self.component.get_active_idx() {
                self.ui_tx.send(Action::SetSource(idx));
            }
        };
    }
}

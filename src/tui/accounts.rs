use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;

use super::{
    component::{Component, ComponentProps},
    list::ListComponent,
};

pub struct Accounts {
    component: ListComponent<String>,
    ui_tx: UnboundedSender<Action>,
}

impl Accounts {
    pub fn new(items: &Vec<String>, ui_tx: UnboundedSender<Action>) -> Accounts {
        Accounts {
            component: ListComponent::new("Accounts".to_string(), items.to_owned()),
            ui_tx,
        }
    }
    pub fn render(
        &mut self,
        f: &mut ratatui::prelude::Frame,
        area: ratatui::prelude::Rect,
        props: Option<ComponentProps>,
    ) {
        self.component.render(f, area, props)
    }
    pub fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) {
        self.component.handle_key_events(key);
        if let crossterm::event::KeyCode::Enter = key.code {
            if let Some(idx) = self.component.get_active_idx() {
                self.ui_tx.send(Action::SetAccount(idx));
            }
        };
    }
}

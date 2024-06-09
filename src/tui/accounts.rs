use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;

use super::{
    component::{Component, ComponentProps},
    list::ListComponent,
    region::Region,
};

pub struct Accounts {
    component: ListComponent<String>,
    ui_tx: UnboundedSender<Action>,
    region: Region,
}

pub trait WithPopup {
    fn open_popup(&mut self);
    fn close_popup(&mut self);
    fn is_popup_open(&self) -> bool;
}

impl WithPopup for Accounts {
    fn open_popup(&mut self) {
        self.region.open = true;
    }

    fn close_popup(&mut self) {
        self.region.open = false;
    }

    fn is_popup_open(&self) -> bool {
        self.region.open
    }
}

impl Accounts {
    pub fn new(
        items: &Vec<String>,
        active_account: &Option<String>,
        region: String,
        ui_tx: UnboundedSender<Action>,
    ) -> Accounts {
        Accounts {
            component: ListComponent::new(
                "Accounts".to_string(),
                items.to_owned(),
                active_account.to_owned(),
            ),
            ui_tx: ui_tx.clone(),
            region: Region::new(region, ui_tx.clone()),
        }
    }
}

impl Component for Accounts {
    fn render(
        &mut self,
        f: &mut ratatui::prelude::Frame,
        area: ratatui::prelude::Rect,
        props: Option<ComponentProps>,
    ) {
        self.component.render(f, area, props);
        if self.is_popup_open() {
            self.region
                .render(f, area, Some(ComponentProps { selected: true }));
        }
    }
    fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) {
        if self.is_popup_open() {
            //handle events region component
            self.region.handle_key_events(key);
        } else {
            self.component.handle_key_events(key);
            if let crossterm::event::KeyCode::Enter = key.code {
                if let Some(idx) = self.component.get_active_idx() {
                    self.ui_tx.send(Action::SetAccount(idx));
                }
            };
            if let crossterm::event::KeyCode::Char('r') = key.code {
                self.open_popup();
            }
        }
    }
}

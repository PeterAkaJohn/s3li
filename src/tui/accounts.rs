mod add_property;
mod edit;

use edit::EditAccount;
use tokio::sync::mpsc::UnboundedSender;

use crate::{action::Action, logger::LOGGER, providers::AccountMap};

use super::{
    component::{Component, ComponentProps},
    list::ListComponent,
    popup::WithPopup,
    region::Region,
};

pub struct Accounts {
    component: ListComponent<String>,
    account_map: AccountMap,
    edit_popup: EditAccount,
    region_popup: Region,
    ui_tx: UnboundedSender<Action>,
}

impl Accounts {
    pub fn new(
        items: &Vec<String>,
        account_map: AccountMap,
        region: String,
        active_account: &Option<String>,
        ui_tx: UnboundedSender<Action>,
    ) -> Accounts {
        Accounts {
            component: ListComponent::new(
                "Accounts".to_string(),
                items.to_owned(),
                active_account.to_owned(),
            ),
            account_map,
            edit_popup: EditAccount::new(ui_tx.clone()),
            region_popup: Region::new(region, ui_tx.clone()),
            ui_tx: ui_tx.clone(),
        }
    }
    pub fn is_locked(&self) -> bool {
        self.edit_popup.is_popup_open() || self.region_popup.is_popup_open()
    }
}

impl Component for Accounts {
    fn render(
        &mut self,
        f: &mut ratatui::prelude::Frame,
        area: ratatui::prelude::Rect,
        props: Option<ComponentProps>,
    ) {
        self.component.render(f, area, props.clone());

        if self.edit_popup.is_popup_open() {
            self.edit_popup.render(f, area, props.clone());
        }
        if self.region_popup.is_popup_open() {
            self.region_popup.render(f, area, props.clone());
        }
    }
    fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) {
        if self.edit_popup.is_popup_open() {
            self.edit_popup.handle_key_events(key)
        }
        if self.region_popup.is_popup_open() {
            self.region_popup.handle_key_events(key)
        }
        self.component.handle_key_events(key);
        match key.code {
            crossterm::event::KeyCode::Char('e') => {
                let account_value = self.component.get_selected_item_value();
                let account_properties = self.account_map.get(account_value);
                if let Some(account_values) = account_properties {
                    self.edit_popup
                        .update_properties(account_value.to_string(), account_values.to_owned());
                    self.edit_popup.open_popup();
                }
            }
            crossterm::event::KeyCode::Char('r') => {
                self.region_popup.open_popup();
            }
            crossterm::event::KeyCode::Enter => {
                if let Some(idx) = self.component.get_active_idx() {
                    let _ = match self.ui_tx.send(Action::SetAccount(idx)) {
                        Ok(_) => LOGGER.info(&format!("send set account with idx {idx}")),
                        Err(_) => LOGGER.info(&format!("failed to set account with idx {idx}")),
                    };
                }
            }
            _ => {}
        };
    }
}

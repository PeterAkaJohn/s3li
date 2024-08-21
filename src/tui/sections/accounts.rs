mod add_property;
mod edit;
mod region;

use crossterm::event::KeyModifiers;
use edit::EditAccount;
use region::Region;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action,
    logger::LOGGER,
    providers::AccountMap,
    tui::{
        components::{
            list::ListComponent,
            popup::WithPopup,
            traits::{Component, ComponentProps, WithList},
        },
        key_event::{EventListeners, ExecuteEventListener, S3liKeyEvent},
    },
};

pub struct Accounts {
    component: ListComponent<String>,
    account_map: AccountMap,
    edit_popup: EditAccount,
    region_popup: Region,
    ui_tx: UnboundedSender<Action>,
    listeners: Vec<EventListeners<Self>>,
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
            listeners: Self::register_listeners(),
        }
    }
    pub fn is_locked(&self) -> bool {
        self.edit_popup.is_popup_open() || self.region_popup.is_popup_open()
    }

    fn register_listeners() -> Vec<EventListeners<Self>> {
        vec![
            EventListeners::KeyEvent((
                S3liKeyEvent::new(vec![(
                    crossterm::event::KeyCode::Char('e'),
                    KeyModifiers::NONE,
                )]),
                Self::edit_properties,
            )),
            EventListeners::KeyEvent((
                S3liKeyEvent::new(vec![(
                    crossterm::event::KeyCode::Char('r'),
                    KeyModifiers::NONE,
                )]),
                Self::edit_region,
            )),
            EventListeners::KeyEvent((
                S3liKeyEvent::new(vec![(crossterm::event::KeyCode::Enter, KeyModifiers::NONE)]),
                Self::confirm_selection,
            )),
        ]
    }

    fn edit_properties(&mut self) {
        if self.component.get_list_state_selected().is_some() {
            let account_value = self.component.get_selected_item_value();
            let account_properties = self.account_map.get(account_value);
            if let Some(account_values) = account_properties {
                self.edit_popup
                    .update_properties(account_value.to_string(), account_values.to_owned());
                self.edit_popup.open_popup();
            }
        }
    }
    fn edit_region(&mut self) {
        self.region_popup.open_popup();
    }
    fn confirm_selection(&mut self) {
        if let Some(idx) = self.component.get_active_idx() {
            let _ = match self.ui_tx.send(Action::SetAccount(idx)) {
                Ok(_) => LOGGER.info(&format!("send set account with idx {idx}")),
                Err(_) => LOGGER.info(&format!("failed to set account with idx {idx}")),
            };
        }
    }
}

impl ExecuteEventListener for Accounts {
    fn get_event_listeners(&self) -> &Vec<EventListeners<Self>> {
        &self.listeners
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
            self.edit_popup.handle_key_events(key);
            return;
        }
        if self.region_popup.is_popup_open() {
            self.region_popup.handle_key_events(key);
            return;
        }
        self.component.handle_key_events(key);
        self.execute(key);
    }
}

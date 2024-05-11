use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;

use super::{
    component::{Component, ComponentProps},
    list::ListComponent,
};

pub struct Sources<'a> {
    component: ListComponent<'a, &'a str>,
    ui_tx: UnboundedSender<Action>,
}

impl<'a> Sources<'a> {
    pub fn new(items: Vec<&'a str>, ui_tx: UnboundedSender<Action>) -> Sources<'a> {
        Sources {
            component: ListComponent::new("Sources", items),
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
        self.component.handle_key_events(key)
    }
}

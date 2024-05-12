

use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;

use super::{
    component::{Component, ComponentProps},
    simple::SimpleComponent,
};

#[derive(Debug)]
pub struct Explorer<'a> {
    selected_file: Option<&'a str>,
    ui_tx: UnboundedSender<Action>,
    component: SimpleComponent<'a>,
}

impl<'a> Explorer<'a> {
    pub fn new(ui_tx: UnboundedSender<Action>) -> Self {
        Self {
            selected_file: None,
            ui_tx,
            component: SimpleComponent::new("Explorer"),
        }
    }
}

impl Explorer<'_> {
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

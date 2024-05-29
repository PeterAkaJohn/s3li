use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::List,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;

use super::{
    component::{Component, ComponentProps, WithContainer},
    simple::SimpleComponent,
};

#[derive(Debug)]
pub struct Explorer {
    selected_file: Option<String>,
    files: Vec<String>,
    folders: Vec<String>,
    ui_tx: UnboundedSender<Action>,
    component: SimpleComponent,
}

impl Explorer {
    pub fn new(files: &Vec<String>, folders: &Vec<String>, ui_tx: UnboundedSender<Action>) -> Self {
        Self {
            selected_file: None,
            files: files.to_owned(),
            folders: folders.to_owned(),
            ui_tx,
            component: SimpleComponent::new("Explorer".to_string()),
        }
    }
}

impl WithContainer<'_> for Explorer {}

impl Explorer {
    pub fn render(
        &mut self,
        f: &mut ratatui::prelude::Frame,
        area: ratatui::prelude::Rect,
        props: Option<ComponentProps>,
    ) {
        let block = self.with_container("Explorer", &props);
        let folders = self.folders.clone();
        let files = self.files.clone();
        let inner_area = block.inner(area);

        let list_folders = List::new(folders).block(block);
        f.render_widget(list_folders, area);
    }

    pub fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) {
        self.component.handle_key_events(key)
    }
}

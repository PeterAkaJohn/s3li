use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action,
    logger::LOGGER,
    store::explorer::{FileTree, TreeItem},
};

use super::{
    component::{Component, ComponentProps, WithContainer},
    simple::SimpleComponent,
};

#[derive(Debug)]
pub struct Explorer {
    selected_file: Option<String>,
    file_tree: Vec<TreeItem>,
    ui_tx: UnboundedSender<Action>,
    component: SimpleComponent,
}

impl Explorer {
    pub fn new(file_tree: Option<FileTree>, ui_tx: UnboundedSender<Action>) -> Self {
        Self {
            selected_file: None,
            file_tree: file_tree.map(|ft| ft.tree_to_vec()).unwrap_or_default(),
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
        let inner_area = block.inner(area);

        LOGGER.info(&format!("{:#?}", self.file_tree));

        // let list_folders = List::new(folders).block(block);
        // f.render_widget(list_folders, area);
    }

    pub fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) {
        self.component.handle_key_events(key)
    }
}

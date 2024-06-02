use crossterm::event::KeyEventKind;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{List, ListItem, ListState},
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action,
    logger::LOGGER,
    store::explorer::{FileTree, Folder, TreeItem},
};

use super::{
    component::{Component, ComponentProps, WithContainer},
    list::WithList,
};

#[derive(Debug)]
pub struct Explorer {
    list_state: ListState,
    selected_file: Option<String>,
    file_tree: Vec<TreeItem>,
    ui_tx: UnboundedSender<Action>,
}

impl Explorer {
    pub fn new(file_tree: Option<FileTree>, ui_tx: UnboundedSender<Action>) -> Self {
        Self {
            list_state: ListState::default(),
            selected_file: None,
            file_tree: file_tree.map(|ft| ft.tree_to_vec()).unwrap_or_default(),
            ui_tx,
        }
    }
}

impl WithContainer<'_> for Explorer {}

impl WithList for Explorer {
    fn get_list_items_len(&self) -> usize {
        self.file_tree.len()
    }

    fn get_list_state_selected(&self) -> Option<usize> {
        self.list_state.selected()
    }

    fn set_selected(&mut self, idx: Option<usize>) {
        self.list_state.select(idx);
    }
}

impl Component for Explorer {
    fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        if self.file_tree.is_empty() {
            return;
        }
        match key.code {
            crossterm::event::KeyCode::Enter => {
                let selected_item = self
                    .get_list_state_selected()
                    .and_then(|idx| self.file_tree.get(idx));
                if let Some(tree_item) = selected_item {
                    match tree_item {
                        TreeItem::Folder(folder, _) => self
                            .ui_tx
                            .send(Action::SetExplorerFolder(folder.name.clone()))
                            .expect("should not fail"),
                        TreeItem::File(_, _) => {}
                    }
                };
            }
            crossterm::event::KeyCode::Esc => {
                self.unselect();
            }
            crossterm::event::KeyCode::Up | crossterm::event::KeyCode::Char('k') => {
                self.select_previous();
            }
            crossterm::event::KeyCode::Down | crossterm::event::KeyCode::Char('j') => {
                self.select_next();
            }

            _ => {}
        };
    }

    fn render(
        &mut self,
        f: &mut ratatui::prelude::Frame,
        area: ratatui::prelude::Rect,
        props: Option<ComponentProps>,
    ) {
        LOGGER.info(&format!("{:#?}", self.file_tree));
        let container = self.with_container("Explorer", &props);

        // let list_folders = List::new(folders).block(block);
        // f.render_widget(list_folders, area);
        let active_style = Style::default().fg(Color::Green).bg(Color::LightBlue);
        let default_style = Style::default().fg(Color::White);
        let list_items = self
            .file_tree
            .iter()
            .filter(|tree_item| {
                if let TreeItem::Folder(folder, _) = tree_item {
                    return folder.name != *"/";
                }
                true
            })
            .map(|tree_item| {
                LOGGER.info(&format!("{:?}", tree_item));
                let label = match tree_item {
                    TreeItem::Folder(folder, _) => format!("▶ {}", folder.relative_name),
                    TreeItem::File(file, _) => format!("{}", file.relative_name),
                };
                ListItem::new(Line::from(Span::styled(
                    tree_item.with_indentation(label),
                    default_style,
                )))
            })
            .collect::<Vec<ListItem>>();

        let list = List::new(list_items)
            .block(container)
            .scroll_padding(2)
            .highlight_style(active_style)
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

        f.render_stateful_widget(list, area, &mut self.list_state);
    }
}

trait WithIndentation {
    fn with_indentation(&self, label: String) -> String;
}

impl WithIndentation for TreeItem {
    fn with_indentation(&self, label: String) -> String {
        match self {
            TreeItem::Folder(folder, _) => {
                if folder.depth > 0 {
                    let mut new_label = " ".repeat(folder.depth);
                    new_label.push_str(&label);
                    LOGGER.info(&format!("{}", &new_label));
                    return new_label;
                }
                label
            }
            TreeItem::File(file, _) => {
                if file.depth > 0 {
                    let mut new_label = "   ".repeat(file.depth);
                    new_label.push_str(&label);
                    LOGGER.info(&format!("{}", &new_label));
                    return new_label;
                }
                label
            }
        }
    }
}

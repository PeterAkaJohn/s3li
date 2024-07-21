mod download;

use crossterm::event::KeyEventKind;
use download::Download;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{List, ListItem, ListState},
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action,
    store::{
        explorer::{FileTree, Folder, TreeItem},
        state::DashboardComponents,
    },
    tui::components::{
        list::WithList,
        popup::WithPopup,
        traits::{Component, ComponentProps, WithContainer},
    },
};

#[derive(Debug)]
pub struct Explorer {
    list_state: ListState,
    selected_file: Option<String>,
    file_tree: Vec<TreeItem>,
    ui_tx: UnboundedSender<Action>,
    current_folder_idx: Option<usize>,
    download_component: Download,
}

impl Explorer {
    pub fn new(
        file_tree: Option<FileTree>,
        current_folder: Option<Folder>,
        ui_tx: UnboundedSender<Action>,
    ) -> Self {
        let file_tree_vec = file_tree
            .map(|ft| ft.tree_to_vec())
            .unwrap_or_default()
            .into_iter()
            .filter(|tree_item| {
                if let TreeItem::Folder(folder, _) = tree_item {
                    return folder.name != *"/";
                }
                true
            })
            .collect::<Vec<TreeItem>>();

        let current_folder_idx = current_folder.and_then(|val| {
            file_tree_vec.iter().position(|tree_item| {
                if let TreeItem::Folder(folder, _) = tree_item {
                    *folder.name == val.name
                } else {
                    false
                }
            })
        });

        let mut list_state = ListState::default();
        list_state.select(current_folder_idx);
        Self {
            list_state,
            selected_file: None,
            file_tree: file_tree_vec,
            ui_tx: ui_tx.clone(),
            current_folder_idx,
            download_component: Download::new(ui_tx.clone()),
        }
    }
    pub fn set_active_idx(&mut self, active_idx: Option<usize>) {
        self.current_folder_idx = active_idx;
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
        if self.download_component.is_popup_open() {
            self.download_component.handle_key_events(key);
            return;
        }
        match key.code {
            crossterm::event::KeyCode::Enter => {
                let selected_idx = self.get_list_state_selected();
                self.set_active_idx(selected_idx);
                let selected_item = selected_idx.and_then(|idx| self.file_tree.get(idx));
                if let Some(tree_item) = selected_item {
                    match tree_item {
                        TreeItem::Folder(_, _) => self
                            .ui_tx
                            .send(Action::SetExplorerFolder(tree_item.clone()))
                            .expect("should not fail"),
                        TreeItem::File(_, _) => {}
                    }
                };
            }
            crossterm::event::KeyCode::Esc => {
                self.unselect();
                self.set_active_idx(None);
                let _ = self
                    .ui_tx
                    .send(Action::SetSelectedComponent(DashboardComponents::Sources));
            }
            crossterm::event::KeyCode::Up | crossterm::event::KeyCode::Char('k') => {
                self.select_previous();
            }
            crossterm::event::KeyCode::Down | crossterm::event::KeyCode::Char('j') => {
                self.select_next();
            }
            crossterm::event::KeyCode::Char('d') => {
                let selected_idx = self.get_list_state_selected();
                let selected_item = selected_idx.and_then(|idx| self.file_tree.get(idx));
                if let Some(TreeItem::File(file, _)) = selected_item {
                    self.download_component.init(file.name.clone());
                }
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
        let container = self.with_container("Explorer", &props);

        let active_style = Style::default().fg(Color::Green).bg(Color::LightBlue);
        let default_style = Style::default().fg(Color::White);
        let mut file_tree_iterator = self.file_tree.iter().peekable();

        let mut list_items: Vec<ListItem> = vec![];
        if let Some(ComponentProps { selected: true }) = props {
            while let Some(tree_item) = file_tree_iterator.next() {
                let label = match tree_item {
                    TreeItem::Folder(folder, _) => {
                        let arrow_char = match file_tree_iterator.peek() {
                            Some(TreeItem::Folder(_, Some(parent)))
                            | Some(TreeItem::File(_, Some(parent))) => {
                                if parent.name == folder.name {
                                    "▼"
                                } else {
                                    "▶"
                                }
                            }
                            _ => "▶",
                        };
                        format!("{} {}", arrow_char, folder.relative_name)
                    }
                    TreeItem::File(file, _) => file.relative_name.to_string(),
                };
                list_items.push(ListItem::new(Line::from(Span::styled(
                    tree_item.with_indentation(label),
                    default_style,
                ))));
            }
        }
        let list = List::new(list_items)
            .block(container)
            .scroll_padding(2)
            .highlight_style(active_style)
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);
        f.render_stateful_widget(list, area, &mut self.list_state);

        if self.download_component.is_popup_open() {
            self.download_component.render(f, area, props);
        }
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
                    return new_label;
                }
                label
            }
            TreeItem::File(file, _) => {
                if file.depth > 0 {
                    let mut new_label = " ".repeat(file.depth + 2);
                    new_label.push_str(&label);
                    return new_label;
                }
                label
            }
        }
    }
}

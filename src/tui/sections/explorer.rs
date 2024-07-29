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
    logger::LOGGER,
    store::{
        explorer::{FileTree, Folder, TreeItem},
        state::DashboardComponents,
    },
    tui::components::{
        functions::add_white_space_till_width_if_needed,
        list::ListMode,
        popup::WithPopup,
        traits::{
            Component, ComponentProps, Selection, SelectionDirection, WithBlockSelection,
            WithContainer, WithList, WithMultiSelection,
        },
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
    mode: ListMode,
    selection: Vec<usize>,
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
            selection: vec![],
            mode: ListMode::Normal,
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

impl WithBlockSelection for Explorer {
    fn start_selection(&mut self, idx: usize) {
        self.mode = ListMode::Selection;
        self.selection = vec![idx];
    }
    fn end_selection(&mut self) {
        self.mode = ListMode::Normal;
        self.selection = vec![];
    }

    fn get_selection(&self) -> Option<Selection> {
        let selection = self.selection.iter().min().zip(self.selection.iter().max());

        selection.map(|val| (*val.0, *val.1))
    }

    fn set_selection(&mut self, selection: Option<Selection>) {
        if let Some((min, max)) = selection {
            let range: Vec<usize> = (min..=max).collect();
            self.selection = range;
        }
    }
}

impl WithMultiSelection for Explorer {
    fn get_multi_selection(&self) -> Vec<usize> {
        self.selection.clone()
    }

    fn set_multi_selection(&mut self, selection: Vec<usize>) {
        self.selection = selection;
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
            crossterm::event::KeyCode::Char(' ') => {
                self.mode = ListMode::Multi;
                let current_idx = self.get_list_state_selected();
                if let Some(idx) = current_idx {
                    self.toggle_selection(idx);
                }
                if self.selection.is_empty() {
                    self.mode = ListMode::Normal;
                }
                LOGGER.info(&format!("selection: {:?}", self.selection));
            }
            crossterm::event::KeyCode::Esc
                if matches!(self.mode, ListMode::Selection | ListMode::Multi) =>
            {
                self.end_selection();
            }
            crossterm::event::KeyCode::Char('v') => {
                if matches!(self.mode, ListMode::Normal) {
                    let current_idx = self.get_list_state_selected();
                    if let Some(idx) = current_idx {
                        self.start_selection(idx);
                    }
                } else {
                    self.end_selection();
                }
            }
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
                if matches!(self.mode, ListMode::Selection) {
                    self.resize_selection(SelectionDirection::Up);
                } else {
                    self.select_previous();
                }
            }
            crossterm::event::KeyCode::Down | crossterm::event::KeyCode::Char('j') => {
                if matches!(self.mode, ListMode::Selection) {
                    self.resize_selection(SelectionDirection::Down);
                } else {
                    self.select_next();
                }
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

        let list_items = if let Some(ComponentProps { selected: true }) = props {
            let mut file_tree_iterator = self.file_tree.iter().enumerate().peekable();
            let mut items = vec![];
            while let Some((idx, tree_item)) = file_tree_iterator.next() {
                let label = match tree_item {
                    TreeItem::Folder(folder, _) => {
                        let arrow_char = match file_tree_iterator.peek() {
                            Some((_, TreeItem::Folder(_, Some(parent))))
                            | Some((_, TreeItem::File(_, Some(parent)))) => {
                                if parent.name == folder.name {
                                    "▼"
                                } else {
                                    "▶"
                                }
                            }
                            _ => "▶",
                        };
                        format!(
                            "{} {}",
                            arrow_char,
                            add_white_space_till_width_if_needed(
                                &folder.relative_name,
                                area.width.into()
                            )
                        )
                    }
                    TreeItem::File(file, _) => {
                        add_white_space_till_width_if_needed(&file.relative_name, area.width.into())
                    }
                };
                let is_selected = self.selection.contains(&idx);
                items.push(ListItem::new(Line::from(Span::styled(
                    tree_item.with_indentation(label),
                    if is_selected {
                        active_style
                    } else {
                        default_style
                    },
                ))));
            }
            items
        } else {
            vec![]
        };
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

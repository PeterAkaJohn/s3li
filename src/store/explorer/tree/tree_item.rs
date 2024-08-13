use super::{File, Folder};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TreeItem {
    Folder(Folder, Option<Folder>),
    File(File, Option<Folder>),
}

impl TreeItem {
    pub fn name(&self) -> &str {
        match self {
            TreeItem::Folder(folder, _) => &folder.relative_name,
            TreeItem::File(file, _) => &file.relative_name,
        }
    }

    pub fn pop_name_char(&mut self) {
        match self {
            TreeItem::Folder(folder, _) => folder.relative_name.pop(),
            TreeItem::File(file, _) => file.relative_name.pop(),
        };
    }
    pub fn push_name_char(&mut self, val: char) {
        match self {
            TreeItem::Folder(folder, _) => folder.relative_name.push(val),
            TreeItem::File(file, _) => file.relative_name.push(val),
        };
    }
}

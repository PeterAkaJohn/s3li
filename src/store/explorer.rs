use std::{
    fmt::Display,
    str::FromStr,
    sync::{Arc, Mutex},
};

#[derive(Debug, Default, Clone, Eq, Hash, PartialEq)]
pub struct Folder {
    pub name: String,
}

impl FromStr for Folder {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            name: s.to_string(),
        })
    }
}
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct File {
    pub name: String,
}

impl FromStr for File {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            name: s.to_string(),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TreeItem {
    Folder(Folder, Option<Folder>),
    File(File, Option<Folder>),
}

#[derive(Debug, Default, Clone)]
pub struct Explorer {
    pub selected_folder: Option<Folder>,
    pub file_tree: FileTree,
}

impl Explorer {
    pub fn new() -> Self {
        Self {
            selected_folder: None,
            file_tree: FileTree::new(
                "/".parse().expect("root folder cannot fail"),
                vec![],
                vec![],
            ),
        }
    }
    pub fn update_folder(
        &self,
        current_folder: Folder,
        current_folder_files: Vec<File>,
        folders: Vec<Folder>,
    ) -> Option<TreeNode> {
        let current_node = &self.file_tree.search(current_folder);
        match current_node {
            Some(current_node) => {
                let mut current_node = current_node.lock().unwrap();
                current_node.files = current_folder_files;
                current_node.children = folders
                    .iter()
                    .map(|folder| {
                        FileTree::create_tree_node(Node::new(folder.clone(), vec![], vec![]))
                    })
                    .collect();
            }
            None => {}
        };
        current_node.clone()
    }
}

#[derive(Debug, Default)]
pub struct Node {
    folder: Folder,
    children: Vec<TreeNode>,
    files: Vec<File>,
}

impl Node {
    pub fn new(folder: Folder, children: Vec<TreeNode>, files: Vec<File>) -> Self {
        Self {
            folder,
            children,
            files,
        }
    }

    fn add_files(&mut self, files: Vec<File>) {
        self.files = files;
    }

    fn add_children(&mut self, nodes: Vec<TreeNode>) {
        self.children = nodes
    }
}

type TreeNode = Arc<Mutex<Node>>;

#[derive(Clone, Default, Debug)]
pub struct FileTree {
    root: TreeNode,
}

impl Display for FileTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.root)
    }
}

impl FileTree {
    pub fn new(root_folder: Folder, folders: Vec<Folder>, files: Vec<File>) -> Self {
        Self {
            root: Arc::new(Mutex::new(Node {
                folder: root_folder,
                children: folders
                    .iter()
                    .map(|folder| {
                        FileTree::create_tree_node(Node::new(folder.clone(), vec![], vec![]))
                    })
                    .collect(),
                files,
            })),
        }
    }

    fn create_tree_node(node: Node) -> TreeNode {
        Arc::new(Mutex::new(node))
    }

    pub fn get_root(&self) -> &TreeNode {
        &self.root
    }

    pub fn insert(self, node: Node, folder_to_find: Folder) -> Self {
        let parent_node = search_tree(self.root.clone(), &folder_to_find);
        if let Some(parent_node) = parent_node {
            parent_node
                .lock()
                .unwrap()
                .children
                .push(FileTree::create_tree_node(node));
        }
        self
    }

    pub fn search(&self, folder_to_find: Folder) -> Option<TreeNode> {
        search_tree(self.root.clone(), &folder_to_find)
    }

    pub fn tree_to_vec(self) -> Vec<TreeItem> {
        let mut tree_items: Vec<TreeItem> = vec![];
        nodes_to_vec(self.root, &mut tree_items, None);

        tree_items
    }
}

pub fn nodes_to_vec(
    source: TreeNode,
    tree_items: &mut Vec<TreeItem>,
    parent_folder: Option<Folder>,
) {
    let node = source.lock().unwrap();
    tree_items.push(TreeItem::Folder(node.folder.to_owned(), parent_folder));

    for child_tree in &node.children {
        nodes_to_vec(child_tree.clone(), tree_items, Some(node.folder.clone()));
    }

    tree_items.append(
        &mut node
            .files
            .iter()
            .map(|file| TreeItem::File(file.to_owned(), Some(node.folder.to_owned())))
            .collect(),
    );
}

pub fn search_tree(source: TreeNode, folder_to_find: &Folder) -> Option<TreeNode> {
    if source.lock().unwrap().folder == *folder_to_find {
        return Some(source);
    }

    let mut node: Option<TreeNode> = None;
    for child_tree in &source.lock().unwrap().children {
        let maybe_node = search_tree(child_tree.to_owned(), folder_to_find);
        if maybe_node.is_some() {
            node = maybe_node;
        }
    }
    node
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use crate::store::explorer::TreeItem;

    use super::{File, FileTree, Node};

    #[test]
    fn test_nodes_to_vec() {
        let one = Arc::new(Mutex::new(Node {
            folder: "one"
                .parse()
                .expect("test folder should be always available"),
            children: vec![],
            files: vec![
                File {
                    name: "one1".to_string(),
                },
                File {
                    name: "one2".to_string(),
                },
            ],
        }));
        let two = Node {
            folder: "two"
                .parse()
                .expect("test folder should be always available"),
            children: vec![one],
            files: vec![
                File {
                    name: "two1".to_string(),
                },
                File {
                    name: "two2".to_string(),
                },
            ],
        };
        let file_tree = FileTree::new(
            "root"
                .parse()
                .expect("test folder should be always available"),
            vec![],
            vec![],
        );

        let file_tree = file_tree.insert(two, "root".parse().expect("cannot fail"));
        let result = file_tree.tree_to_vec();

        assert_eq!(
            result.first().unwrap(),
            &TreeItem::Folder("root".parse().unwrap(), None)
        );
        assert_eq!(
            result.get(1).unwrap(),
            &TreeItem::Folder("two".parse().unwrap(), Some("root".parse().unwrap()))
        );
        assert_eq!(
            result.get(2).unwrap(),
            &TreeItem::Folder("one".parse().unwrap(), Some("two".parse().unwrap()))
        );
        assert_eq!(
            result.get(3).unwrap(),
            &TreeItem::File("one1".parse().unwrap(), Some("one".parse().unwrap()))
        );
        assert_eq!(
            result.get(4).unwrap(),
            &TreeItem::File("one2".parse().unwrap(), Some("one".parse().unwrap()))
        );
        assert_eq!(
            result.get(5).unwrap(),
            &TreeItem::File("two1".parse().unwrap(), Some("two".parse().unwrap()))
        );
        assert_eq!(
            result.get(6).unwrap(),
            &TreeItem::File("two2".parse().unwrap(), Some("two".parse().unwrap()))
        )
    }

    #[test]
    fn test_find_node() {
        let one = Arc::new(Mutex::new(Node {
            folder: "one"
                .parse()
                .expect("test folder should be always available"),
            children: vec![],
            files: vec![],
        }));
        let two = Node {
            folder: "two"
                .parse()
                .expect("test folder should be always available"),
            children: vec![one],
            files: vec![],
        };
        let file_tree = FileTree::new(
            "root"
                .parse()
                .expect("test folder should be always available"),
            vec![],
            vec![],
        );

        let result = file_tree
            .insert(
                two,
                "root"
                    .parse()
                    .expect("test folder should be always available"),
            )
            .search(
                "one"
                    .parse()
                    .expect("test folder should be always available"),
            );

        assert!(result.is_some());
        assert_eq!(
            result.unwrap().lock().unwrap().folder,
            "one"
                .parse()
                .expect("test folder should be always available")
        );
    }
}

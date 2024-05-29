use std::{
    cell::RefCell,
    fmt::Display,
    rc::Rc,
    sync::{Arc, Mutex},
};

#[derive(Debug, Default, Clone, Eq, Hash, PartialEq)]
pub struct Folder {
    pub name: String,
}
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct File {
    pub name: String,
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
                Folder {
                    name: "/".to_string(),
                },
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
    use std::{
        cell::RefCell,
        rc::Rc,
        sync::{Arc, Mutex},
    };

    use super::{FileTree, Folder, Node};

    #[test]
    fn test_find_node() {
        let one = Arc::new(Mutex::new(Node {
            folder: Folder {
                name: "one".to_string(),
            },
            children: vec![],
            files: vec![],
        }));
        let two = Node {
            folder: Folder {
                name: "two".to_string(),
            },
            children: vec![one],
            files: vec![],
        };
        let file_tree = FileTree::new(
            Folder {
                name: "root".to_string(),
            },
            vec![],
            vec![],
        );

        println!("{}", file_tree);
        let result = file_tree
            .insert(
                two,
                Folder {
                    name: "root".to_string(),
                },
            )
            .search(Folder {
                name: "one".to_string(),
            });

        assert!(result.is_some());
        assert_eq!(
            result.unwrap().lock().unwrap().folder,
            Folder {
                name: "one".to_string()
            }
        );
    }
}

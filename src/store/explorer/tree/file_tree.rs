use std::{fmt::Display, sync::Arc};

use std::sync::Mutex;

use super::{File, Folder, TreeItem};

#[derive(Debug, Default)]
pub struct Node {
    folder: Folder,
    pub children: Vec<TreeNode>,
    pub files: Vec<File>,
}

impl Node {
    pub fn new(folder: Folder, children: Vec<TreeNode>, files: Vec<File>) -> Self {
        Self {
            folder,
            children,
            files,
        }
    }
}

pub type TreeNode = Arc<Mutex<Node>>;

impl From<Node> for TreeNode {
    fn from(value: Node) -> Self {
        Arc::new(Mutex::new(value))
    }
}

impl From<Folder> for TreeNode {
    fn from(value: Folder) -> Self {
        Arc::new(Mutex::new(Node::new(value, vec![], vec![])))
    }
}

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
                children: folders.into_iter().map(|folder| folder.into()).collect(),
                files,
            })),
        }
    }

    pub fn get_root(&self) -> &TreeNode {
        &self.root
    }

    pub fn insert(self, node: Node, folder_to_find: Folder) -> Self {
        let parent_node = search_tree(self.root.clone(), &folder_to_find);
        if let Some(parent_node) = parent_node {
            parent_node.lock().unwrap().children.push(node.into());
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

    use crate::store::explorer::{File, Folder, TreeItem};

    use super::{FileTree, Node};

    #[test]
    fn test_folder_relative_name() {
        let folder: Folder = "/".parse().unwrap();
        assert_eq!(folder.name, "/".to_string());
        assert_eq!(folder.relative_name, "".to_string());
        assert_eq!(folder.depth, 0);
        let folder: Folder = "test/".parse().unwrap();
        assert_eq!(folder.name, "test/".to_string());
        assert_eq!(folder.relative_name, "test".to_string());
        assert_eq!(folder.depth, 0);
        let folder: Folder = "test/something/".parse().unwrap();
        assert_eq!(folder.name, "test/something/".to_string());
        assert_eq!(folder.relative_name, "something".to_string());
        assert_eq!(folder.depth, 1);
        let folder: Folder = "test/something/amazing/".parse().unwrap();
        assert_eq!(folder.name, "test/something/amazing/".to_string());
        assert_eq!(folder.relative_name, "amazing".to_string());
        assert_eq!(folder.depth, 2);
    }

    #[test]
    fn test_file_relative_name() {
        let file: File = "test.test".parse().unwrap();
        assert_eq!(file.name, "test.test".to_string());
        assert_eq!(file.relative_name, "test.test".to_string());
        assert_eq!(file.depth, 0);
        let file: File = "test/test.test".parse().unwrap();
        assert_eq!(file.name, "test/test.test".to_string());
        assert_eq!(file.relative_name, "test.test".to_string());
        assert_eq!(file.depth, 1);
        let file: File = "test/something/test.test".parse().unwrap();
        assert_eq!(file.name, "test/something/test.test".to_string());
        assert_eq!(file.relative_name, "test.test".to_string());
        assert_eq!(file.depth, 2);
        let file: File = "test/something/amazing/test.test".parse().unwrap();
        assert_eq!(file.name, "test/something/amazing/test.test".to_string());
        assert_eq!(file.relative_name, "test.test".to_string());
        assert_eq!(file.depth, 3);
    }

    #[test]
    fn test_nodes_to_vec() {
        let one = Arc::new(Mutex::new(Node {
            folder: "one"
                .parse()
                .expect("test folder should be always available"),
            children: vec![],
            files: vec!["one1".parse().unwrap(), "one2".parse().unwrap()],
        }));
        let two = Node {
            folder: "two"
                .parse()
                .expect("test folder should be always available"),
            children: vec![one],
            files: vec!["two1".parse().unwrap(), "two2".parse().unwrap()],
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

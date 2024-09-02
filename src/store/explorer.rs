mod tree;
use anyhow::{anyhow, Result};
pub use tree::{File, FileTree, Folder, TreeItem};

use core::panic;
use std::sync::Arc;

use tokio::sync::Mutex as TokioMutex;
use tree::TreeNode;

use crate::providers::AwsClient;

#[derive(Debug, Clone)]
pub struct Explorer {
    pub selected_folder: Option<Folder>,
    pub file_tree: FileTree,
    client: Arc<TokioMutex<AwsClient>>,
}

impl Explorer {
    pub fn new(client: Arc<TokioMutex<AwsClient>>) -> Self {
        Self {
            selected_folder: None,
            file_tree: FileTree::new(
                "/".parse().expect("root folder cannot fail"),
                vec![],
                vec![],
            ),
            client,
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
                current_node.children = folders.into_iter().map(|folder| folder.into()).collect();
            }
            None => {}
        };
        current_node.clone()
    }

    pub async fn create_file_tree(&mut self, bucket: &str) -> Result<bool> {
        let (files, folders) = self
            .client
            .lock()
            .await
            .list_objects_one_level(bucket, None)
            .await?;
        let file_tree = FileTree::new(
            "/".parse().expect("root_folder initialization cannot fail"),
            folders
                .iter()
                .map(|folder| folder.parse().expect("folder creation cannot fail"))
                .collect(),
            files
                .iter()
                .map(|file_key| file_key.parse().expect("file creation cannot fail"))
                .collect(),
        );
        self.selected_folder = Some("/".parse().expect("root_folder initialization cannot fail"));
        self.file_tree = file_tree;
        Ok(true)
    }

    pub async fn update_file_tree(&mut self, bucket: &str, tree_item: &TreeItem) -> Result<Folder> {
        let new_selected_folder = if let TreeItem::Folder(folder, parent) = tree_item {
            if self.selected_folder == Some(folder.clone()) {
                // this means that we need to remove child of selected
                // folder so we return the parent
                parent
                    .as_ref()
                    .filter(|parent_folder| parent_folder.name != "/")
            } else {
                Some(folder)
            }
        } else {
            panic!("cannot be a file tree_item");
        };
        let (files, folders) = self
            .client
            .lock()
            .await
            .list_objects_one_level(
                bucket,
                new_selected_folder
                    .map(|folder| folder.name.clone())
                    .as_deref(),
            )
            .await?;
        if let Some(folder) = new_selected_folder {
            self.update_folder(
                folder.clone(),
                files
                    .iter()
                    .map(|file_key| file_key.parse().expect("file creation cannot fail"))
                    .collect(),
                folders
                    .iter()
                    .map(|new_folder| new_folder.parse().expect("folder creation cannot fail"))
                    .collect(),
            );
        } else {
            let file_tree = FileTree::new(
                "/".parse().expect("root_folder initialization cannot fail"),
                folders
                    .iter()
                    .map(|folder| folder.parse().expect("folder creation cannot fail"))
                    .collect(),
                files
                    .iter()
                    .map(|file_key| file_key.parse().expect("file creation cannot fail"))
                    .collect(),
            );
            self.file_tree = file_tree;
        }
        match new_selected_folder {
            Some(folder) => Ok(folder.clone()),
            None => Err(anyhow!("Error during update_file_tree")),
        }
    }
}

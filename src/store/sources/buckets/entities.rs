use anyhow::{Ok, Result};

use crate::{
    providers::AwsClient,
    store::{
        explorer::{File, Folder, TreeItem},
        sources::traits::Downloadable,
    },
};

pub struct BucketFile {
    key: String,
    name: String,
}

impl BucketFile {
    pub fn new(key: String, name: String) -> Self {
        Self { key, name }
    }

    pub fn from_key(full_key: String, parent_key: &str, parent_name: &str) -> Self {
        let file_name = full_key
            .chars()
            .skip(parent_key.chars().count())
            .collect::<String>();
        let file_name = format!("{}/{}", parent_name, file_name);

        BucketFile {
            key: full_key,
            name: file_name,
        }
    }
}

pub struct BucketFolder {
    key: String,
    name: String,
}

impl BucketFolder {
    pub fn new(key: String, name: String) -> Self {
        Self { key, name }
    }
}

impl Downloadable for BucketFile {
    async fn download(&self, client: AwsClient, source: String) -> Result<bool> {
        client.download_file(&source, &self.key, &self.name).await
    }
}

impl Downloadable for BucketFolder {
    async fn download(&self, client: AwsClient, source: String) -> Result<bool> {
        let files_in_folder = client.list_objects(&source, &self.key).await;

        let files_to_download = files_in_folder
            .into_iter()
            .map(|file| BucketFile::from_key(file, &self.key, &self.name))
            .collect::<Vec<_>>();

        for file_to_download in files_to_download {
            file_to_download
                .download(client.clone(), source.clone())
                .await;
        }

        Ok(true)
    }
}

pub enum BucketItem {
    BucketFile(BucketFile),
    BucketFolder(BucketFolder),
}

impl From<TreeItem> for BucketItem {
    fn from(value: TreeItem) -> Self {
        match value {
            TreeItem::Folder(
                Folder {
                    name,
                    relative_name,
                    ..
                },
                _,
            ) => BucketItem::BucketFolder(BucketFolder::new(name, relative_name)),
            TreeItem::File(
                File {
                    name,
                    relative_name,
                    ..
                },
                _,
            ) => BucketItem::BucketFile(BucketFile::new(name, relative_name)),
        }
    }
}

impl Downloadable for BucketItem {
    async fn download(&self, client: AwsClient, source: String) -> Result<bool> {
        match self {
            BucketItem::BucketFile(file) => file.download(client, source.to_string()).await,
            BucketItem::BucketFolder(folder) => folder.download(client, source.to_string()).await,
        }
    }
}

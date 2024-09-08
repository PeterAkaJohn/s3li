use anyhow::{Context, Result};
use futures::future::join_all;

use crate::{
    providers::ProviderClient,
    store::{
        explorer::{File, Folder, TreeItem},
        sources::traits::{DownloadResult, Downloadable},
    },
};

#[derive(Clone)]
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
    async fn download(
        &self,
        client: impl ProviderClient + Clone + 'static,
        source: String,
    ) -> Result<DownloadResult> {
        let mut result = DownloadResult::default();
        let download_result = client
            .download_file(&source, &self.key, &self.name)
            .await
            .with_context(|| {
                format!(
                    "File with key {} and name {} failed to download",
                    self.key, self.name
                )
            });
        result.append_to_result(self.key.clone(), download_result);
        Ok(result)
    }
}

impl Downloadable for BucketFolder {
    async fn download(
        &self,
        client: impl ProviderClient + Clone + 'static,
        source: String,
    ) -> Result<DownloadResult> {
        let files_in_folder = client.list_objects(&source, &self.key).await?;

        let files_to_download = files_in_folder
            .into_iter()
            .map(|file| BucketFile::from_key(file, &self.key, &self.name))
            .collect::<Vec<_>>();

        let operations = files_to_download
            .into_iter()
            .map(|file_to_download| {
                let client_cloned = client.clone();
                let source_cloned = source.clone();
                tokio::task::spawn(async move {
                    file_to_download
                        .clone()
                        .download(client_cloned, source_cloned)
                        .await
                })
            })
            .collect::<Vec<_>>();

        let results = join_all(operations).await;
        let all_results: Result<Vec<DownloadResult>> =
            results.into_iter().map(|res| res?).collect();

        all_results.map(|val| {
            val.into_iter()
                .fold(DownloadResult::default(), |acc, res| acc.merge_results(res))
        })
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
    async fn download(
        &self,
        client: impl ProviderClient + Clone + 'static,
        source: String,
    ) -> Result<DownloadResult> {
        match self {
            BucketItem::BucketFile(file) => file.download(client, source.to_string()).await,
            BucketItem::BucketFolder(folder) => folder.download(client, source.to_string()).await,
        }
    }
}

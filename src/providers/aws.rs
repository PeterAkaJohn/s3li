mod credentials;

use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    io::Write,
};

use anyhow::{anyhow, Result};
use aws_config::{profile::ProfileFileCredentialsProvider, BehaviorVersion, Region};
use aws_sdk_s3::Client;
pub use credentials::{AuthProperties, Credentials};

use crate::logger::LOGGER;

#[derive(Debug, Clone)]
pub struct AwsClient {
    account: String,
    client: Client,
    credentials: Credentials,
    pub region: String,
}

pub type AccountMap = HashMap<String, HashMap<String, Option<String>>>;

impl AwsClient {
    pub async fn new() -> Self {
        let config = aws_config::load_defaults(BehaviorVersion::v2024_03_28()).await;
        let client = Client::new(&config);
        Self {
            account: "default".to_string(),
            region: "eu-central-1".to_string(),
            credentials: Credentials::default(),
            client,
        }
    }

    async fn refresh_client(&mut self) {
        let credentials_provider = ProfileFileCredentialsProvider::builder()
            .profile_name(&self.account)
            .build();
        let config = aws_config::defaults(BehaviorVersion::v2024_03_28())
            .credentials_provider(credentials_provider)
            .region(Region::new(self.region.clone()))
            .load()
            .await;
        self.client = Client::new(&config);
    }

    pub async fn switch_account(&mut self, new_account: &str) {
        self.account = new_account.to_string();
        self.refresh_client().await;
    }
    pub async fn change_region(&mut self, region: String) {
        self.region = region.clone();
        self.refresh_client().await;
    }

    pub async fn list_buckets(&self) -> Result<Vec<String>> {
        let resp = self.client.list_buckets().send().await.map_err(|e| {
            LOGGER.info(&format!("Error during list_buckets {:?}", e));
            anyhow!("Error during list_buckets")
        })?;
        let buckets = resp
            .buckets()
            .iter()
            .map(|buck| buck.name().unwrap_or_default().to_string())
            .collect::<Vec<_>>();
        Ok(buckets)
    }

    pub fn list_accounts(&self) -> Result<AccountMap> {
        let credentials = Credentials::default();
        match credentials.list_accounts() {
            Ok(accounts) => Ok(accounts
                .iter()
                .map(|account| {
                    let account_properties = credentials.get_properties(account);
                    (account.to_string(), account_properties)
                })
                .collect::<AccountMap>()),
            Err(e) => {
                LOGGER.info(&format!("Error during list_accounts {:?}", e));
                Err(anyhow!("Error during list_accounts. Try again."))
            }
        }
    }

    pub fn update_account(
        &mut self,
        account: &str,
        properties: AuthProperties,
    ) -> Result<AccountMap> {
        match self.credentials.update_account(account, properties) {
            Ok(_) => self.list_accounts(),
            Err(e) => {
                LOGGER.info(&format!("Error during update_account {:?}", e));
                Err(anyhow!("Error during update_account"))
            }
        }
    }

    pub async fn download_file(
        &self,
        bucket: &str,
        file_key: &str,
        file_name: &str,
    ) -> Result<bool> {
        let parent_folder_to_create = file_name
            .split('/')
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .skip(1)
            .rev()
            .collect::<Vec<_>>();
        let parent_folder_to_create = parent_folder_to_create.join("/");
        let _ = fs::create_dir_all(parent_folder_to_create);
        let mut destination_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_name)?;

        let object = self
            .client
            .get_object()
            .bucket(bucket)
            .key(file_key)
            .send()
            .await?;
        let bytes = object.body.collect().await?.into_bytes();
        destination_file.write_all(&bytes)?;
        Ok(true)
    }

    pub async fn list_objects(&self, bucket: &str, prefix: &str) -> Vec<String> {
        let mut response = self
            .client
            .list_objects_v2()
            .bucket(bucket)
            .prefix(prefix)
            .into_paginator()
            .send();
        let mut result_files: Vec<String> = vec![];
        while let Some(result) = response.next().await {
            match result {
                Ok(objects) => {
                    let mut files = objects
                        .contents()
                        .iter()
                        .map(|val| val.key().unwrap_or("Unknown").to_owned())
                        .collect::<Vec<_>>();
                    result_files.append(&mut files);
                }
                Err(_) => break,
            };
        }
        result_files
    }

    pub async fn list_objects_one_level(
        &self,
        bucket: &str,
        current_folder: Option<&str>,
    ) -> (Vec<String>, Vec<String>) {
        let prefix = current_folder.unwrap_or("");
        let mut response = self
            .client
            .list_objects_v2()
            .bucket(bucket)
            .prefix(prefix)
            .delimiter("/")
            .into_paginator()
            .send();
        let mut result_files: Vec<String> = vec![];
        let mut result_folders: Vec<String> = vec![];
        while let Some(result) = response.next().await {
            match result {
                Ok(objects) => {
                    let mut folders = objects
                        .common_prefixes()
                        .iter()
                        .map(|val| val.prefix().unwrap_or("Unknown").to_owned())
                        .collect::<Vec<_>>();
                    let mut files = objects
                        .contents()
                        .iter()
                        .map(|val| val.key().unwrap_or("Unknown").to_owned())
                        .collect::<Vec<_>>();
                    result_files.append(&mut files);
                    result_folders.append(&mut folders);
                }
                Err(_) => break,
            };
        }
        (result_files, result_folders)
    }
}

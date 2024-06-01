use anyhow::Result;
use aws_config::{
    meta::region::RegionProviderChain, profile::ProfileFileCredentialsProvider,
    retry::ProvideErrorKind, BehaviorVersion,
};
use aws_sdk_s3::Client;
use dirs::home_dir;
use ini::ini;

use crate::logger::LOGGER;

pub struct AwsClient {
    account: String,
    client: Client,
}

impl AwsClient {
    pub async fn new() -> Self {
        let config = aws_config::load_defaults(BehaviorVersion::v2024_03_28()).await;
        let client = Client::new(&config);
        Self {
            account: "default".to_string(),
            client,
        }
    }
    pub async fn switch_account(&mut self, new_account: &str) {
        self.account = new_account.to_string();
        let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
        let credentials_provider = ProfileFileCredentialsProvider::builder()
            .profile_name(&self.account)
            .build();
        let config = aws_config::defaults(BehaviorVersion::v2024_03_28())
            .credentials_provider(credentials_provider)
            .region("us-east-1")
            .load()
            .await;
        self.client = Client::new(&config);
    }

    pub async fn list_buckets(&self) -> Result<Vec<String>> {
        let resp = self.client.list_buckets().send().await?;
        let buckets = resp
            .buckets()
            .iter()
            .map(|buck| buck.name().unwrap_or_default().to_string())
            .collect::<Vec<_>>();
        Ok(buckets)
    }

    pub fn list_accounts() -> Vec<String> {
        let home = home_dir();
        let credentials_path = if let Some(home) = home {
            let mut home_string = home.into_os_string();
            home_string.push("/.aws/credentials");
            home_string
        } else {
            panic!("Failed to find the home dir");
        };
        let credentials_path = match credentials_path.to_str() {
            Some(credentials_path) => credentials_path,
            None => panic!("Failed to get credentials file"),
        };
        let credentials_file = ini!(safe credentials_path);
        match credentials_file {
            Ok(credentials_file) => credentials_file
                .keys()
                .map(|key| key.to_string())
                .collect::<Vec<String>>(),
            Err(_) => {
                panic!("Credentials file does not exists")
            }
        }
    }

    pub async fn list_objects(
        &self,
        bucket: &str,
        current_folder: Option<&str>,
    ) -> (Vec<String>, Vec<String>) {
        let prefix = current_folder.unwrap_or("");
        LOGGER.info(&format!("prefix {}", prefix));
        let mut response = self
            .client
            .list_objects_v2()
            .bucket(bucket)
            .prefix(prefix)
            .delimiter("/")
            .max_keys(100)
            .into_paginator()
            .send();
        let mut result_files: Vec<String> = vec![];
        let mut result_folders: Vec<String> = vec![];
        while let Some(result) = response.next().await {
            match result {
                Ok(objects) => {
                    LOGGER.info(&format!("{:#?}", objects));
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

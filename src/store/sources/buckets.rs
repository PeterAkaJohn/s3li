use std::sync::Arc;

use anyhow::{anyhow, Result};
use tokio::sync::Mutex;

use crate::providers::AwsClient;

use super::traits::WithSources;

#[derive(Debug, Clone)]
pub struct Buckets {
    available_sources: Vec<String>,
    active_source: Option<String>,
    client: Arc<Mutex<AwsClient>>,
}

impl Buckets {
    pub fn new(client: Arc<Mutex<AwsClient>>) -> Self {
        Self {
            available_sources: vec![],
            active_source: None,
            client: client.clone(),
        }
    }
}

impl WithSources for Buckets {
    fn set_source_with_idx(&mut self, idx: usize) -> &Option<String> {
        self.active_source = self.available_sources.get(idx).map(|val| val.to_string());
        &self.active_source
    }

    fn get_active_source(&self) -> &Option<String> {
        &self.active_source
    }

    fn get_available_sources(&self) -> &Vec<String> {
        &self.available_sources
    }

    async fn update_available_sources(&mut self) {
        let buckets = self.client.lock().await.list_buckets().await;
        let sources = if let Ok(buckets) = buckets {
            buckets
        } else {
            vec![]
        };
        self.available_sources = sources;
    }

    async fn download_file(&self, key: &str, file_name: &str) -> Result<bool> {
        let selected_bucket = self.get_active_source();
        if let Some(bucket) = selected_bucket {
            self.client
                .lock()
                .await
                .download_file(bucket, key, file_name)
                .await
        } else {
            Err(anyhow!("selected bucket was empty!!!"))
        }
    }

    async fn download_folder(&self, key: &str, new_folder_name: &str) -> Result<bool> {
        let selected_bucket = self.get_active_source();
        if let Some(bucket) = selected_bucket {
            self.client
                .lock()
                .await
                .download_folder(bucket, key, new_folder_name)
                .await
        } else {
            Err(anyhow!("selected bucket was empty!!!"))
        }
    }
}

pub mod entities;

use std::sync::Arc;

use tokio::sync::Mutex;

use crate::providers::AwsClient;

use super::traits::{DownloadResult, Downloadable, WithSources};

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
}

impl Buckets {
    pub async fn download(&self, items: Vec<impl Downloadable>) -> DownloadResult {
        let mut result = DownloadResult::default();
        for item in items {
            let download_result = item
                .download(
                    self.client.lock().await.clone(),
                    self.active_source.clone().unwrap(),
                )
                .await;
            result = result.merge_results(download_result);
        }
        result
    }
}

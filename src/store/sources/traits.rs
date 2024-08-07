use anyhow::Result;

use crate::providers::AwsClient;

pub trait WithSources {
    fn set_source_with_idx(&mut self, idx: usize) -> &Option<String>;
    fn get_active_source(&self) -> &Option<String>;
    fn get_available_sources(&self) -> &Vec<String>;
    async fn update_available_sources(&mut self);
}

#[derive(Default)]
pub struct DownloadResult {
    pub results: Vec<Result<bool>>,
}

impl DownloadResult {
    pub fn append_to_result(&mut self, result: Result<bool>) {
        self.results.push(result);
    }
    pub fn merge_results(mut self, other_result: DownloadResult) -> Self {
        self.results.extend(other_result.results);
        self
    }
}

pub trait Downloadable {
    async fn download(&self, client: AwsClient, source: String) -> DownloadResult;
}

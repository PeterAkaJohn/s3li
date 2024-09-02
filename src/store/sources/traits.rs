use anyhow::Result;

use crate::providers::AwsClient;

pub trait WithSources {
    fn set_source_with_idx(&mut self, idx: usize) -> &Option<String>;
    fn get_active_source(&self) -> &Option<String>;
    fn get_available_sources(&self) -> &Vec<String>;
    async fn update_available_sources(&mut self) -> Result<&Vec<String>>;
}

#[derive(Default, Debug)]
pub enum DownloadResultStatus {
    Successful,
    Failed,
    Partial,
    #[default]
    Pending,
}

#[derive(Default, Debug)]
pub struct DownloadResult {
    pub status: DownloadResultStatus,
    pub results: Vec<(String, Result<bool>)>,
}

impl DownloadResult {
    fn update_status_on_append(&mut self, is_err: bool) {
        match self.status {
            DownloadResultStatus::Successful if is_err => {
                self.status = DownloadResultStatus::Partial
            }
            DownloadResultStatus::Failed if !is_err => self.status = DownloadResultStatus::Partial,
            DownloadResultStatus::Pending => {
                self.status = if is_err {
                    DownloadResultStatus::Failed
                } else {
                    DownloadResultStatus::Successful
                }
            }
            _ => {}
        };
    }
    fn update_status_on_merge(&mut self, other_status: DownloadResultStatus) {
        match (&self.status, other_status) {
            (DownloadResultStatus::Successful, DownloadResultStatus::Failed)
            | (DownloadResultStatus::Successful, DownloadResultStatus::Partial)
            | (DownloadResultStatus::Failed, DownloadResultStatus::Successful)
            | (DownloadResultStatus::Failed, DownloadResultStatus::Partial)
            | (DownloadResultStatus::Partial, DownloadResultStatus::Successful)
            | (DownloadResultStatus::Partial, DownloadResultStatus::Failed) => {
                self.status = DownloadResultStatus::Partial
            }
            _ => {}
        }
    }
    pub fn append_to_result(&mut self, file_key: String, result: Result<bool>) {
        self.update_status_on_append(result.is_err());
        self.results.push((file_key, result));
    }
    pub fn merge_results(mut self, other_result: DownloadResult) -> Self {
        self.update_status_on_merge(other_result.status);
        self.results.extend(other_result.results);
        self
    }
}

pub trait Downloadable {
    async fn download(&self, client: AwsClient, source: String) -> Result<DownloadResult>;
}

use anyhow::Result;

use crate::providers::AwsClient;

pub trait WithSources {
    fn set_source_with_idx(&mut self, idx: usize) -> &Option<String>;
    fn get_active_source(&self) -> &Option<String>;
    fn get_available_sources(&self) -> &Vec<String>;
    async fn update_available_sources(&mut self);
}

pub trait Downloadable {
    async fn download(&self, client: AwsClient, source: String) -> Result<bool>;
}

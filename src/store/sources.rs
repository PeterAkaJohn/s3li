use anyhow::Result;
use buckets::Buckets;
pub use traits::WithSources;
use traits::{DownloadResult, Downloadable};

pub mod buckets;
pub mod traits;

#[derive(Debug, Clone)]
pub enum Sources {
    Buckets(Buckets),
}

impl WithSources for Sources {
    fn set_source_with_idx(&mut self, idx: usize) -> &Option<String> {
        match self {
            Sources::Buckets(buckets) => buckets.set_source_with_idx(idx),
        }
    }

    fn get_active_source(&self) -> &Option<String> {
        match self {
            Sources::Buckets(buckets) => buckets.get_active_source(),
        }
    }

    fn get_available_sources(&self) -> &Vec<String> {
        match self {
            Sources::Buckets(buckets) => buckets.get_available_sources(),
        }
    }

    async fn update_available_sources(&mut self) -> Result<&Vec<String>> {
        match self {
            Sources::Buckets(buckets) => buckets.update_available_sources().await,
        }
    }
}

impl Sources {
    pub async fn download(&self, items: Vec<impl Downloadable>) -> Result<DownloadResult> {
        match self {
            Sources::Buckets(buckets) => buckets.download(items).await,
        }
    }
}

use buckets::Buckets;
pub use traits::WithSources;

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

    async fn update_available_sources(&mut self) {
        match self {
            Sources::Buckets(buckets) => buckets.update_available_sources().await,
        };
    }

    async fn download_file(&self, key: &str, file_name: &str) -> anyhow::Result<bool> {
        match self {
            Sources::Buckets(buckets) => buckets.download_file(key, file_name).await,
        }
    }
    async fn download_folder(&self, key: &str, new_folder_name: &str) -> anyhow::Result<bool> {
        match self {
            Sources::Buckets(buckets) => buckets.download_folder(key, new_folder_name).await,
        }
    }
}

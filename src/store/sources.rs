use buckets::Buckets;
pub use traits::WithSources;

pub mod buckets;
pub mod traits;

#[derive(Debug, Clone)]
pub enum Sources {
    Buckets(Buckets),
}

impl Default for Sources {
    fn default() -> Self {
        Self::Buckets(Buckets::default())
    }
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

    fn set_available_sources(&mut self, sources: Vec<String>) {
        match self {
            Sources::Buckets(buckets) => buckets.set_available_sources(sources),
        }
    }
}

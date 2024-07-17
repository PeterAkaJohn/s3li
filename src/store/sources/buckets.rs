use super::traits::WithSources;

#[derive(Debug, Default, Clone)]
pub struct Buckets {
    available_sources: Vec<String>,
    active_source: Option<String>,
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

    fn set_available_sources(&mut self, sources: Vec<String>) {
        self.available_sources = sources;
    }
}

#[derive(Debug, Default, Clone)]
pub struct Sources {
    pub available_sources: Vec<String>,
    pub active_source: Option<String>,
}

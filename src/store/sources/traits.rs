pub trait WithSources {
    fn set_source_with_idx(&mut self, idx: usize) -> &Option<String>;
    fn get_active_source(&self) -> &Option<String>;
    fn get_available_sources(&self) -> &Vec<String>;
    fn set_available_sources(&mut self, sources: Vec<String>);
}

use std::str::FromStr;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct File {
    pub name: String,
    pub relative_name: String,
    pub depth: usize,
}
impl FromStr for File {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let full_name_split = s.split('/').collect::<Vec<&str>>();
        let relative_name = full_name_split.last().unwrap_or(&"/").to_string();
        let depth = full_name_split.len();
        let depth = if depth > 1 { depth - 1 } else { 0 };
        Ok(Self {
            name: s.to_string(),
            relative_name,
            depth,
        })
    }
}

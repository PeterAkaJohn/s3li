use std::str::FromStr;

#[derive(Debug, Default, Clone, Eq, Hash, PartialEq)]
pub struct Folder {
    pub name: String,
    pub relative_name: String,
    pub depth: usize,
}

impl FromStr for Folder {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (relative_name, depth) = if s.ends_with('/') {
            // this is a folder
            let full_name_split = s.split('/').collect::<Vec<&str>>();
            let depth = full_name_split.len();
            let depth = if depth > 2 { depth - 2 } else { 0 };
            let folder_relative_name = match full_name_split[..] {
                [.., relative_name_item, _] => relative_name_item.to_string(),
                _ => {
                    // root folder case
                    panic!("relative name should always be available")
                }
            };
            (folder_relative_name, depth)
        } else {
            // this is a file
            (s.split('/').last().unwrap_or(s).to_string(), 0)
        };
        Ok(Self {
            name: s.to_string(),
            relative_name,
            depth,
        })
    }
}

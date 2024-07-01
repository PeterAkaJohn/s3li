use crate::{providers::AuthProperties, store::explorer::TreeItem};

#[derive(Debug, Clone)]
pub enum Action {
    Quit,
    Tick,
    Render,
    Key(String),
    SetSource(usize),
    SetAccount(usize),
    SetExplorerFolder(TreeItem),
    DownloadFile(String, String),
    ChangeRegion(String),
    RefreshCredentials,
    EditCredentials(String, AuthProperties),
}

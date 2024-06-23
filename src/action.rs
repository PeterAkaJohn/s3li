use crate::{providers::AuthProperties, store::explorer::TreeItem};

#[derive(Debug)]
pub enum Action {
    Quit,
    Tick,
    Render,
    Key(String),
    SetSource(usize),
    SetAccount(usize),
    SetExplorerFolder(TreeItem),
    DownloadFile(String),
    ChangeRegion(String),
    RefreshCredentials,
    EditCredentials(usize, AuthProperties),
}

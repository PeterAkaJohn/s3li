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
    ChangeRegion(String),
    RefreshCredentials,
    EditCredentials(usize, AuthProperties),
}

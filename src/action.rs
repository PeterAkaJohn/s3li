use crate::{
    providers::AuthProperties,
    store::{explorer::TreeItem, state::DashboardComponents},
};

#[derive(Debug, Clone)]
pub enum Action {
    Quit,
    Tick,
    Render,
    Key(String),
    SetSource(usize),
    SetAccount(usize),
    SetExplorerFolder(TreeItem),
    Download(Vec<TreeItem>),
    ChangeRegion(String),
    RefreshCredentials,
    EditCredentials(String, AuthProperties),
    SetSelectedComponent(DashboardComponents),
    CycleSelectedComponent,
    DismissLastAlert,
}

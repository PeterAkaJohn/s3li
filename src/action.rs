use crate::{
    providers::AuthProperties,
    store::{
        explorer::{FileToDownload, TreeItem},
        state::DashboardComponents,
    },
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
    DownloadFile(Vec<FileToDownload>),
    ChangeRegion(String),
    RefreshCredentials,
    EditCredentials(String, AuthProperties),
    SetSelectedComponent(DashboardComponents),
    CycleSelectedComponent,
}

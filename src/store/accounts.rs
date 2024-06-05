#[derive(Debug, Default, Clone)]
pub struct Accounts {
    pub available_accounts: Vec<String>,
    pub active_account: Option<String>,
    pub region: String,
}

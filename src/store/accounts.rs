use crate::providers::AccountMap;

#[derive(Debug, Default, Clone)]
pub struct Accounts {
    pub account_map: AccountMap,
    pub available_accounts: Vec<String>,
    pub active_account: Option<String>,
    pub region: String,
}

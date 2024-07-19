use crate::providers::AccountMap;

#[derive(Debug, Default, Clone)]
pub struct Accounts {
    pub account_map: AccountMap,
    pub available_accounts: Vec<String>,
    pub active_account: Option<String>,
    pub region: String,
}

impl Accounts {
    pub fn new(
        account_map: AccountMap,
        available_accounts: Vec<String>,
        active_account: Option<String>,
        region: String,
    ) -> Self {
        Self {
            account_map,
            available_accounts,
            active_account,
            region,
        }
    }
}

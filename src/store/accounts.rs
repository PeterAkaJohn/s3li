use std::sync::Arc;

use anyhow::Result;
use tokio::sync::Mutex;

use crate::providers::{AccountMap, AuthProperties, AwsClient};

#[derive(Debug, Clone)]
pub struct Accounts {
    pub account_map: AccountMap,
    pub available_accounts: Vec<String>,
    pub active_account: Option<String>,
    pub region: String,
    client: Arc<Mutex<AwsClient>>,
}

impl Accounts {
    pub async fn new(
        client: Arc<Mutex<AwsClient>>,
        active_account: Option<String>,
    ) -> Result<Self> {
        let account_map = client.clone().lock().await.list_accounts()?;
        let available_accounts: Vec<String> =
            Accounts::extract_available_account_from_account_map(&account_map);
        Ok(Self {
            client: client.clone(),
            account_map,
            available_accounts,
            active_account,
            region: client.clone().lock().await.region.clone(),
        })
    }

    pub async fn set_account(&mut self, account_idx: usize) {
        let account = self
            .available_accounts
            .get(account_idx)
            .map(|val| val.as_str())
            .unwrap_or("default");
        self.active_account = Some(account.to_string());
        self.client.lock().await.switch_account(account).await;
    }

    pub async fn change_region(&mut self, new_region: String) {
        self.region = new_region.clone();
        self.client.lock().await.change_region(new_region).await;
    }

    pub async fn refresh_credentials(&mut self) -> Result<()> {
        let account_map = self.client.lock().await.list_accounts()?;

        let mut available_accounts: Vec<String> = account_map
            .clone()
            .keys()
            .map(|key| key.to_string())
            .collect();
        available_accounts.sort();

        self.account_map = account_map;
        self.available_accounts =
            Accounts::extract_available_account_from_account_map(&self.account_map);
        Ok(())
    }

    pub async fn edit_credentials(
        &mut self,
        account: String,
        properties: AuthProperties,
    ) -> Result<()> {
        let account_map = self
            .client
            .lock()
            .await
            .update_account(&account, properties)?;
        self.account_map = account_map;
        self.available_accounts =
            Accounts::extract_available_account_from_account_map(&self.account_map);
        Ok(())
    }

    fn extract_available_account_from_account_map(account_map: &AccountMap) -> Vec<String> {
        let mut available_accounts: Vec<String> =
            account_map.keys().map(|key| key.to_string()).collect();
        available_accounts.sort();
        available_accounts
    }
}

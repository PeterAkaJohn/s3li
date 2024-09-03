use anyhow::Result;

use super::{AccountMap, AuthProperties};

pub trait ProviderClient {
    async fn switch_account(&mut self, new_account: &str);
    async fn change_region(&mut self, region: String);

    async fn list_buckets(&self) -> Result<Vec<String>>;

    fn list_accounts(&self) -> Result<AccountMap>;

    fn update_account(&mut self, account: &str, properties: AuthProperties) -> Result<AccountMap>;

    async fn download_file(&self, bucket: &str, file_key: &str, file_name: &str) -> Result<bool>;

    async fn list_objects(&self, bucket: &str, prefix: &str) -> Result<Vec<String>>;
    async fn list_objects_in_folder(
        &self,
        bucket: &str,
        current_folder: Option<&str>,
    ) -> Result<(Vec<String>, Vec<String>)>;
}

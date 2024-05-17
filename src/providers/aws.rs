use anyhow::Result;
use aws_config::{profile::ProfileFileCredentialsProvider, BehaviorVersion};
use aws_sdk_s3::Client;
use dirs::home_dir;
use ini::ini;

pub struct AwsClient {
    account: String,
    client: Client,
}

impl AwsClient {
    pub async fn new() -> Self {
        let config = aws_config::load_defaults(BehaviorVersion::v2024_03_28()).await;
        let client = Client::new(&config);
        Self {
            account: "default".to_string(),
            client,
        }
    }
    pub async fn switch_account(&mut self, new_account: &str) {
        self.account = new_account.to_string();
        let credentials_provider = ProfileFileCredentialsProvider::builder()
            .profile_name(&self.account)
            .build();
        let config = aws_config::defaults(BehaviorVersion::v2024_03_28())
            .credentials_provider(credentials_provider)
            .load()
            .await;
        self.client = Client::new(&config);
    }

    pub async fn list_buckets(&self) -> Result<Vec<String>> {
        let resp = self.client.list_buckets().send().await?;
        let buckets = resp
            .buckets()
            .iter()
            .map(|buck| buck.name().unwrap_or_default().to_string())
            .collect::<Vec<_>>();
        Ok(buckets)
    }

    pub fn list_accounts() -> Vec<String> {
        let home = home_dir();
        let credentials_path = if let Some(home) = home {
            let mut home_string = home.into_os_string();
            home_string.push("/.aws/credentials");
            home_string
        } else {
            panic!("Failed to find the home dir");
        };
        let credentials_path = match credentials_path.to_str() {
            Some(credentials_path) => credentials_path,
            None => panic!("Failed to get credentials file"),
        };
        let credentials_file = ini!(safe credentials_path);
        match credentials_file {
            Ok(credentials_file) => credentials_file
                .keys()
                .map(|key| key.to_string())
                .collect::<Vec<String>>(),
            Err(_) => {
                panic!("Credentials file does not exists")
            }
        }
    }
}

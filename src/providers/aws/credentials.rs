use anyhow::{anyhow, Ok, Result};
use configparser::ini::Ini;
use dirs::home_dir;

pub struct Credentials {
    file: String,
    accounts: Vec<String>,
}

impl Default for Credentials {
    fn default() -> Self {
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
        Self {
            file: credentials_path.to_string(),
            accounts: vec![],
        }
    }
}

impl Credentials {
    pub fn new(file: Option<String>, accounts: Option<Vec<String>>) -> Self {
        let file = if let Some(file) = file {
            file
        } else {
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
            credentials_path.to_string()
        };
        Self {
            file,
            accounts: accounts.unwrap_or_default(),
        }
    }
    pub fn list_accounts(&self) -> Result<Vec<String>> {
        let mut ini = Ini::new();
        ini.load(self.file.as_str())
            .map(|cred_file| {
                cred_file
                    .keys()
                    .map(|key| key.to_string())
                    .collect::<Vec<String>>()
            })
            .map_err(|e| anyhow!(e))
    }

    pub fn update_account(
        &mut self,
        account_to_update: &str,
        aws_access_key_id: String,
        aws_secret_access_key: String,
        aws_session_token: String,
    ) -> Result<bool> {
        let mut config = Ini::new();
        config.load(self.file.as_str()).map_err(|e| anyhow!(e))?;
        config.set(
            account_to_update,
            "aws_access_key_id",
            Some(aws_access_key_id),
        );
        config.set(
            account_to_update,
            "aws_secret_access_key",
            Some(aws_secret_access_key),
        );
        config.set(
            account_to_update,
            "aws_session_token",
            Some(aws_session_token),
        );
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use dirs::home_dir;

    use super::Credentials;

    #[test]
    fn create_credentials() {
        let mut expected_credentials_file = home_dir().unwrap().into_os_string();
        expected_credentials_file.push("/.aws/credentials");
        let credentials = Credentials::default();
        assert_eq!(
            credentials.file,
            expected_credentials_file.into_string().unwrap()
        );
        assert!(credentials.accounts.is_empty())
    }
}

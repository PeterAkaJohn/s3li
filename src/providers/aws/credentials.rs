use std::collections::HashMap;

use anyhow::{anyhow, Result};
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
        aws_access_key_id: Option<String>,
        aws_secret_access_key: Option<String>,
        aws_session_token: Option<String>,
    ) -> Result<bool> {
        let mut config = Ini::new();
        config.load(self.file.as_str()).map_err(|e| anyhow!(e))?;
        if let Some(value) = aws_access_key_id {
            config.set(account_to_update, "aws_access_key_id", Some(value));
        }
        if let Some(value) = aws_secret_access_key {
            config.set(account_to_update, "aws_secret_access_key", Some(value));
        }

        if let Some(value) = aws_session_token {
            config.set(account_to_update, "aws_session_token", Some(value));
        }

        config.write(&self.file)?;
        Ok(true)
    }

    pub fn get_properties(&self, account_to_get: &str) -> HashMap<String, Option<String>> {
        let mut config = Ini::new();
        let map = config.load(self.file.as_str()).map_err(|e| anyhow!(e));
        match map {
            Ok(actualmap) => actualmap
                .get(account_to_get)
                .unwrap_or(&HashMap::<String, Option<String>>::new())
                .clone(),
            Err(_) => HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use anyhow::{anyhow, Result};
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

    macro_rules! test_resources_folder {
        () => {
            concat!(env!("CARGO_MANIFEST_DIR"), "/resources/test/")
        };
    }

    fn compare_value(
        file_path: &str,
        section: &str,
        entry: &str,
        expected_value: String,
    ) -> Result<()> {
        let mut config = configparser::ini::Ini::new();
        config.load(file_path).map_err(|e| anyhow!(e))?;

        let config_value = config.get(section, entry);
        assert_eq!(config_value, Some(expected_value));
        Ok(())
    }

    #[test]
    fn update_account_credentials() -> Result<()> {
        let test_resources_folder = test_resources_folder!();
        fs::create_dir_all(test_resources_folder)?;
        let config_file_path = format!("{test_resources_folder}/update_account_credentials");

        let config_file = "[test]
aws_access_key_id=test_key
aws_secret_access_key=test_secret
aws_session_token=test_session
";
        fs::write(config_file_path.clone(), config_file)?;

        let mut credentials = Credentials::new(Some(config_file_path.clone()), None);

        credentials.update_account("test", Some("updated_test_key".to_string()), None, None)?;

        compare_value(
            &config_file_path,
            "test",
            "aws_access_key_id",
            "updated_test_key".to_string(),
        )?;
        compare_value(
            &config_file_path,
            "test",
            "aws_secret_access_key",
            "test_secret".to_string(),
        )?;
        compare_value(
            &config_file_path,
            "test",
            "aws_session_token",
            "test_session".to_string(),
        )?;

        credentials.update_account("test", None, Some("updated_test_secret".to_string()), None)?;

        compare_value(
            &config_file_path,
            "test",
            "aws_access_key_id",
            "updated_test_key".to_string(),
        )?;
        compare_value(
            &config_file_path,
            "test",
            "aws_secret_access_key",
            "updated_test_secret".to_string(),
        )?;
        compare_value(
            &config_file_path,
            "test",
            "aws_session_token",
            "test_session".to_string(),
        )?;

        credentials.update_account("test", None, None, Some("updated_test_session".to_string()))?;

        compare_value(
            &config_file_path,
            "test",
            "aws_access_key_id",
            "updated_test_key".to_string(),
        )?;
        compare_value(
            &config_file_path,
            "test",
            "aws_secret_access_key",
            "updated_test_secret".to_string(),
        )?;
        compare_value(
            &config_file_path,
            "test",
            "aws_session_token",
            "updated_test_session".to_string(),
        )?;

        Ok(())
    }
    #[test]
    fn test_get_properties() -> Result<()> {
        let test_resources_folder = test_resources_folder!();
        fs::create_dir_all(test_resources_folder)?;
        let config_file_path = format!("{test_resources_folder}/get_properties");

        let config_file = "[test]
aws_access_key_id=test_key
aws_secret_access_key=test_secret
aws_session_token=test_session
";
        fs::write(config_file_path.clone(), config_file)?;

        let credentials = Credentials::new(Some(config_file_path.clone()), None);

        let properties = credentials.get_properties("test");

        assert!(properties.get("aws_access_key_id").is_some());
        assert!(properties.get("aws_secret_access_key").is_some());
        assert!(properties.get("aws_session_token").is_some());

        assert_eq!(
            properties
                .get("aws_access_key_id")
                .unwrap()
                .clone()
                .unwrap(),
            "test_key"
        );
        assert_eq!(
            properties
                .get("aws_secret_access_key")
                .unwrap()
                .clone()
                .unwrap(),
            "test_secret"
        );
        assert_eq!(
            properties
                .get("aws_session_token")
                .unwrap()
                .clone()
                .unwrap(),
            "test_session"
        );

        Ok(())
    }
}

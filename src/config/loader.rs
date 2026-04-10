use crate::error::{Result, SentryCliError};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub default_org: Option<String>,
    pub server_url: Option<String>,
    pub auth_token: Option<String>,
    pub default_project: Option<String>,
}

impl Config {
    /// Get auth token with priority: CLI flag > env var > config file
    pub fn get_auth_token(&self, cli_override: Option<&str>) -> Result<String> {
        cli_override
            .map(String::from)
            .or_else(|| std::env::var("SENTRY_AUTH_TOKEN").ok())
            .or_else(|| self.auth_token.clone())
            .ok_or_else(|| {
                SentryCliError::Auth(
                    "No auth token found. Set SENTRY_AUTH_TOKEN or configure in config file".into(),
                )
            })
    }

    /// Get server URL with priority: CLI flag > env var > config file > default
    pub fn get_server_url(&self, cli_override: Option<&str>) -> String {
        cli_override
            .map(String::from)
            .or_else(|| std::env::var("SENTRY_SERVER_URL").ok())
            .or_else(|| self.server_url.clone())
            .unwrap_or_else(|| "https://sentry.io".to_string())
    }

    /// Get organization with priority: CLI flag > env var > config file
    pub fn get_org(&self, cli_override: Option<&str>) -> Result<String> {
        cli_override
            .map(String::from)
            .or_else(|| std::env::var("SENTRY_ORG").ok())
            .or_else(|| self.default_org.clone())
            .ok_or_else(|| {
                SentryCliError::Config(
                    "No organization specified. Use --org or configure default_org".into(),
                )
            })
    }
}

/// Get the path to the config file
pub fn config_path() -> PathBuf {
    ProjectDirs::from("", "", "sntry")
        .map(|dirs| dirs.config_dir().join("config.toml"))
        .unwrap_or_else(|| PathBuf::from(".").join("config.toml"))
}

/// Load configuration from file (if exists)
pub fn load_config() -> Config {
    let path = config_path();
    if path.exists() {
        match std::fs::read_to_string(&path) {
            Ok(content) => toml::from_str(&content).unwrap_or_default(),
            Err(_) => Config::default(),
        }
    } else {
        Config::default()
    }
}

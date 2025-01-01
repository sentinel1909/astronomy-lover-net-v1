// src/lib/configuration.rs

// dependencies
use anyhow::{Context, Result};
use shuttle_runtime::SecretStore;

// struct type to represent the app configuration
#[derive(Clone, Debug)]
pub struct AppConfig {
    pub api_key: String,
}

// implement the TryFrom trait for AppConfig
impl TryFrom<SecretStore> for AppConfig {
    type Error = anyhow::Error;

    fn try_from(value: SecretStore) -> Result<Self> {
        let api_key = value.get("NASA_API_KEY").context("NASA API key not set.")?;
        Ok(Self { api_key })
    }
}

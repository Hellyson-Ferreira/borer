use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub token: String,
}

impl AppConfig {
    pub fn new(host: String, token: String) -> Self {
        AppConfig { host, token }
    }

    pub fn save_config(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let toml = toml::to_string_pretty(self)?;
        fs::write(path, toml)?;
        Ok(())
    }

    pub fn load_config(path: &str) -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string(path)?;
        let config: AppConfig = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn delete_config(path: &str) -> Result<(), Box<dyn Error>> {
        fs::remove_file(path)?;
        Ok(())
    }
}

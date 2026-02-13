use crate::core::config::schema::TeamConfig;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub struct ConfigLoader;

impl ConfigLoader {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<TeamConfig> {
        let content = fs::read_to_string(path.as_ref()).context("Failed to read config file")?;
        let config: TeamConfig = toml::from_str(&content).context("Failed to parse config file")?;
        Ok(config)
    }

    pub fn save<P: AsRef<Path>>(config: &TeamConfig, path: P) -> Result<()> {
        let content = toml::to_string_pretty(config).context("Failed to serialize config")?;
        fs::write(path.as_ref(), content).context("Failed to write config file")?;
        Ok(())
    }
}

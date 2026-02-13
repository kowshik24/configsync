use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LocalState {
    pub roles: Vec<String>,
}

impl LocalState {
    pub fn load() -> Result<Self> {
        let path = Self::get_path()?;
        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&path).context("Failed to read local state file")?;
        let state: LocalState = toml::from_str(&content).context("Failed to parse local state file")?;
        Ok(state)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::get_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).context("Failed to create local state directory")?;
        }

        let content = toml::to_string_pretty(self).context("Failed to serialize local state")?;
        fs::write(path, content).context("Failed to write local state file")?;
        Ok(())
    }

    pub fn get_path() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "configsync", "configsync")
            .context("Could not determine project directories")?;
        // Use data_local_dir for machine-specific state (not synced)
        Ok(proj_dirs.data_local_dir().join("state.toml"))
    }

    pub fn add_role(&mut self, role: &str) {
        if !self.roles.contains(&role.to_string()) {
            self.roles.push(role.to_string());
        }
    }
    
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }
}

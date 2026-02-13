use crate::core::config::schema::TeamConfig;
use crate::core::git::repository::GitRepository;
use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::fs;

pub fn init(url: Option<String>) -> Result<()> {
    // ProjectDirs::from("com", "organization", "application")
    let proj_dirs = ProjectDirs::from("com", "configsync", "configsync")
        .context("Could not determine project directories")?;
    let config_dir = proj_dirs.config_dir();

    if config_dir.exists() {
        println!("ConfigSync is already initialized at {:?}", config_dir);
        // In a real app we might want to ask to overwrite or just verify.
        // For MVP, we'll just return if strictly initialized, but maybe we want to re-init or such.
        // For now, let's just warn and continue if empty, or error if not.
        if fs::read_dir(config_dir)?.next().is_some() {
            // It's not empty
            // We can check if it's a git repo
            if config_dir.join(".git").exists() {
                return Ok(());
            }
        }
    }

    fs::create_dir_all(config_dir).context("Failed to create config directory")?;

    if let Some(u) = url {
        println!("Cloning repository from {}...", u);
        GitRepository::clone(&u, config_dir)?;
    } else {
        println!("Initializing new repository at {:?}", config_dir);
        let _repo = GitRepository::init(config_dir)?;

        // Create default config
        let config_path = config_dir.join("team-config.toml");
        if !config_path.exists() {
            let default_config = TeamConfig::default();
            let toml_string = toml::to_string_pretty(&default_config)?;
            fs::write(&config_path, toml_string).context("Failed to write default config")?;
            println!("Created default configuration at {:?}", config_path);
        }
    };

    println!("Initialization complete. Applying configurations...");
    crate::core::engine::apply::apply()?;

    Ok(())
}

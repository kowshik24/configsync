use crate::core::config::loader::ConfigLoader;
use crate::core::fs::symlink;
use anyhow::{Context, Result};
use directories::ProjectDirs;
use shellexpand;
use std::path::PathBuf;

pub fn apply() -> Result<()> {
    // 1. Locate config
    let proj_dirs = ProjectDirs::from("com", "configsync", "configsync")
        .context("Could not determine project directories")?;
    let config_dir = proj_dirs.config_dir();
    let config_path = config_dir.join("team-config.toml");

    if !config_path.exists() {
        println!("No team-config.toml found. Nothing to apply.");
        return Ok(());
    }

    // 2. Load config
    let config = ConfigLoader::load(&config_path)?;

    println!("Applying configurations for team: {}", config.team.name);

    // 2a. Load local state for roles
    use crate::core::state::LocalState;
    let state = LocalState::load().unwrap_or_default();
    println!("Current machine roles: {:?}", state.roles);

    // 3. Iterate files and symlink
    for file in config.files {
        // Role check
        if let Some(ref required_roles) = file.roles {
            if !required_roles.is_empty() {
                let has_role = required_roles.iter().any(|r| state.has_role(r));
                if !has_role {
                    println!("Skipping {:?} (required roles: {:?})", file.source, required_roles);
                    continue;
                }
            }
        }

        let source_path = config_dir.join(&file.source);

        let expanded_dest = shellexpand::tilde(&file.destination);
        let dest_path = PathBuf::from(expanded_dest.into_owned());

        if !source_path.exists() {
            println!(
                "Warning: Source file {:?} does not exist. Skipping.",
                source_path
            );
            continue;
        }

        println!("Linking {:?} <- {:?}", dest_path, source_path);

        // We might want to handle backup here, but for now just try create_symlink
        match symlink::create_symlink(&source_path, &dest_path) {
            Ok(_) => println!("OK"),
            Err(e) => println!("Failed: {}", e),
        }
    }

    Ok(())
}

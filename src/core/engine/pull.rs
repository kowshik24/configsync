use crate::core::git::repository::GitRepository;
use crate::core::engine::apply::apply;
use anyhow::{Result, Context};
use directories::ProjectDirs;

pub fn pull() -> Result<()> {
    let proj_dirs = ProjectDirs::from("com", "configsync", "configsync")
        .context("Could not determine project directories")?;
    let config_dir = proj_dirs.config_dir();

    println!("Opening repository at {:?}", config_dir);
    let repo = GitRepository::open(config_dir)?;

    println!("Pulling changes from remote...");
    match repo.pull() {
        Ok(_) => println!("Successfully pulled changes."),
        Err(e) => println!("Warning: Failed to pull: {}. Proceeding to apply locally.", e),
    }

    println!("Applying configurations...");
    apply()?;

    Ok(())
}

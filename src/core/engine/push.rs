use crate::core::git::repository::GitRepository;
use anyhow::{Result, Context};
use directories::ProjectDirs;

pub fn push() -> Result<()> {
    let proj_dirs = ProjectDirs::from("com", "configsync", "configsync")
        .context("Could not determine project directories")?;
    let config_dir = proj_dirs.config_dir();

    println!("Opening repository at {:?}", config_dir);
    let repo = GitRepository::open(config_dir)?;

    println!("Committing changes...");
    repo.commit_all("Update configurations (configsync)")?;

    println!("Pushing to remote...");
    // Just warn on push failure for MVP (e.g. if no remote or offline)
    match repo.push() {
        Ok(_) => println!("Successfully pushed to remote."),
        Err(e) => println!("Warning: Failed to push to remote: {}. \nChanges are committed locally.", e),
    }

    Ok(())
}

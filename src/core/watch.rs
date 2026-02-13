use crate::core::git::repository::GitRepository;
use anyhow::{Context, Result};
use directories::ProjectDirs;
use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode};
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn start() -> Result<()> {
    let proj_dirs = ProjectDirs::from("com", "configsync", "configsync")
        .context("Could not determine project directories")?;
    let config_dir = proj_dirs.config_dir();

    if !config_dir.exists() {
        anyhow::bail!("ConfigSync not initialized. Run `configsync init` first.");
    }

    println!("Starting ConfigSync Daemon...");
    println!("Watching directory: {:?}", config_dir);

    // Verify git repo exists
    let repo = GitRepository::open(config_dir)?;

    let (tx, rx) = channel();

    // Create a debouncer with 2 seconds timeout
    let mut debouncer =
        new_debouncer(Duration::from_secs(2), tx).context("Failed to create file watcher")?;

    debouncer
        .watcher()
        .watch(config_dir, RecursiveMode::Recursive)
        .context("Failed to start watcher")?;

    // Since this is a CLI tool, we just block on the receiver loop
    for result in rx {
        match result {
            Ok(events) => {
                // Filter out .git changes
                let has_relevant_changes = events
                    .iter()
                    .any(|e| !e.path.components().any(|c| c.as_os_str() == ".git"));

                if has_relevant_changes {
                    println!("Changes detected. Syncing...");
                    match sync_changes(&repo) {
                        Ok(_) => println!("Synced successfully."),
                        Err(e) => eprintln!("Failed to sync: {:#}", e),
                    }
                }
            }
            Err(e) => eprintln!("Watch error: {:?}", e),
        }
    }

    Ok(())
}

fn sync_changes(repo: &GitRepository) -> Result<()> {
    // We attempt to commit. If there are no changes, commit_all might fail or do nothing.
    // Ideally we should check status, but for MVP let's just try.
    // GitRepository::commit_all currently adds all and commits.
    repo.commit_all("Auto-sync: Detected changes")?;
    println!("Pushing to remote...");
    repo.push()?;
    Ok(())
}

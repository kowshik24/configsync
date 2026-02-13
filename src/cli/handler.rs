use crate::cli::args::Commands;
use anyhow::Result;

pub fn handle_command(command: Commands) -> Result<()> {
    match command {
        Commands::Init { url, role } => {
            crate::core::engine::init::init(url, role)?;
            Ok(())
        }
        Commands::Add { path, role } => {
            crate::core::engine::add::add(path, role)?;
            Ok(())
        }
        Commands::Push => {
            crate::core::engine::push::push()?;
            Ok(())
        }
        Commands::Pull => {
            crate::core::engine::pull::pull()?;
            Ok(())
        }
        Commands::Watch => {
            crate::core::watch::start()?;
            Ok(())
        }
        Commands::Secrets { command } => match command {
            crate::cli::args::SecretCommands::Init => {
                let key = crate::core::secret::keys::generate_key()?;
                crate::core::secret::keys::save_key(&key)?;
                println!("Secret key generated at {:?}", crate::core::secret::keys::get_key_path()?);
                Ok(())
            }
            crate::cli::args::SecretCommands::Add { path } => {
                crate::core::engine::add::add_secret(path)?;
                Ok(())
            }
        },
        Commands::History => {
            let config_dir = directories::ProjectDirs::from("com", "configsync", "configsync")
                .unwrap()
                .config_dir()
                .to_path_buf();
            let repo = crate::core::git::repository::GitRepository::open(&config_dir)?;
            repo.log()?;
            Ok(())
        }
        Commands::Undo { commit } => {
            let config_dir = directories::ProjectDirs::from("com", "configsync", "configsync")
                .unwrap()
                .config_dir()
                .to_path_buf();
            let repo = crate::core::git::repository::GitRepository::open(&config_dir)?;
            repo.revert(commit)?;
            // After revert, we might want to auto-apply?
            // "Revert successful. New commit created."
            // The file is reverted in the repo. We need to create the symlink again if it was deleted?
            // Or if content changed, the symlink points to the file in repo, so it should be reflected immediately?
            // Symlink points to valid path. Git updates file content.
            // If git revert deletes the file, then symlink is broken.
            // We should run apply to be safe.
            crate::core::engine::apply::apply()?;
            crate::core::engine::apply::apply()?;
            Ok(())
        }
        Commands::Doctor => {
            crate::core::doctor::check()?;
            Ok(())
        }
    }
}

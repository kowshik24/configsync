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
    }
}

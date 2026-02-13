use crate::cli::args::Commands;
use anyhow::Result;

pub fn handle_command(command: Commands) -> Result<()> {
    match command {
        Commands::Init { url } => {
            crate::core::engine::init::init(url)?;
            Ok(())
        }
        Commands::Add { path } => {
            crate::core::engine::add::add(path)?;
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
    }
}

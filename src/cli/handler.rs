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
    }
}

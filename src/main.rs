use anyhow::Result;
use clap::Parser;
use configsync::cli::{args::Cli, handler::handle_command};

fn main() -> Result<()> {
    let args = Cli::parse();
    handle_command(args.command)?;
    Ok(())
}

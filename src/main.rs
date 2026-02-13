use clap::Parser;
use configsync::cli::{args::Cli, handler::handle_command};
use anyhow::Result;

fn main() -> Result<()> {
    let args = Cli::parse();
    handle_command(args.command)?;
    Ok(())
}

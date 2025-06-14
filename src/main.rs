use anyhow::{Context, Result};
use clap::Parser;
use log::{info, warn};
use std::io::{self};

mod funcs;
mod tests;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
}

fn main() -> Result<()> {
    // Faster printing with buffer
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file `{}`", args.path.display()))?;

    // let pb = indicatif::ProgressBar::new((content.lines().count() - 1) as u64);

    // Logger with Adjusted Verbosity
    env_logger::builder()
        .filter_level(args.verbosity.into())
        .init();

    info!("Starting up");
    warn!("Upps Nothing to warn of");
    let result = funcs::find_matches::find_matches(&content, &args.pattern, handle)?;

    Ok(())
}

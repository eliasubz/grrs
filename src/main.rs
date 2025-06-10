use anyhow::{Context, Result};
use clap::Parser;
use log::{info, warn};
use std::io::{self, Write};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

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
    // let stdout = io::stdout();
    // let mut handle = stdout.lock();
    // writeln!(handle, " {}", line);

    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file `{}`", args.path.display()))?;

    let pb = indicatif::ProgressBar::new((content.lines().count() - 1) as u64);

    // Logger
    // Adjust Verbosity
    env_logger::builder()
        .filter_level(args.verbosity.into())
        .init();

    info!("Starting up");
    warn!("Upps Nothing to warn of");

    for line in content.lines() {
        info!("Searching the next Line");
        thread::sleep(Duration::from_millis(100));
        if line.contains(&args.pattern) {
            warn!("One pattern was found {}", line);
            pb.println(format!("{}", line));
        }
        pb.inc(1);
    }

    Ok(())
}

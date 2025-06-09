use anyhow::{Context, Result};
use clap::Parser;
use log::{info, warn};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
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

    for line in content.lines() {
        thread::sleep(Duration::from_millis(1000));
        if line.contains(&args.pattern) {
            pb.println(format!("{}", line));
        }
        pb.inc(1);
    }

    Ok(())
}

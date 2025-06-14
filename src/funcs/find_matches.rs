use anyhow::Ok;
use anyhow::{Context, Result};
// use indicatif::ProgressBar;
use log::{info, warn};
use std::thread;
use std::time::Duration;

pub fn find_matches(
    content: &str,
    pattern: &str,
    mut writer: impl std::io::Write,
    // pb: &ProgressBar,
) -> Result<()> {
    for line in content.lines() {
        info!("Searching the next Line");
        if line.contains(pattern) {
            thread::sleep(Duration::from_millis(100));
            warn!("One pattern was found {}", line);
            // pb.println(format!("{}", line));
            writeln!(writer, "{}", line)
                .with_context(|| format!("Could not write the line '{}", line))?;
        }
        // pb.inc(1);
    }
    Ok(())
}

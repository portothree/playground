use anyhow::{Context, Result};
use clap::Parser;
use log::{info, trace};
use spinners::{Spinner, Spinners};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::{thread::sleep, time::Duration};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long)]
    pattern: String,
    #[arg(long)]
    path: std::path::PathBuf,
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    trace!("Starting up...");

    let mut sp = Spinner::new(Spinners::Dots9, "Loading...\n".into());
    sleep(Duration::from_secs(1));
    sp.stop();

    let file = File::open(&args.path)
        .with_context(|| format!("Could not read file `{}`", args.path.display()))?;

    info!("Reading file...");

    let mut reader = BufReader::new(file);

    let mut line = String::new();

    while reader.read_line(&mut line)? > 0 {
        if line.contains(&args.pattern) {
            println!("{}", line);
            info!("Pattern found in line: {}", line);
        }
        line.clear();
    }

    Ok(())
}

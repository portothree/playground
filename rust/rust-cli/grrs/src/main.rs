use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long)]
    pattern: String,
    #[arg(long)]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let file = File::open(&args.path)
        .with_context(|| format!("Could not read file `{}`", args.path.display()))?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();

    while reader.read_line(&mut line)? > 0 {
        if line.contains(&args.pattern) {
            print!("{}", line);
        }
        line.clear();
    }

    Ok(())
}

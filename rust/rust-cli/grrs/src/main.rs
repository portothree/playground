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

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    let file = File::open(&args.path)?;
    let mut reader = BufReader::new(file);

    let mut content = String::new();

    reader.read_to_string(&mut content)?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}

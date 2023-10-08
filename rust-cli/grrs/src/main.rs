#![allow(unused)]

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    let file = File::open(&args.path)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    loop {
        let len = reader.read_line(&mut line)?;

        if len == 0 {
            break;
        };

        if line.contains(&args.pattern) {
            println!("{}", line);
        }
        line.clear();
    }

    Ok(())
}
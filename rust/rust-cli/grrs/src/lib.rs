use clap::Parser;
use log::info;
use std::io::BufRead;

pub trait BufReadTrait: BufRead {}

impl<T: BufRead + ?Sized> BufReadTrait for T {}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(long)]
    pub pattern: String,
    #[arg(long)]
    pub path: std::path::PathBuf,
    #[command(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
}

pub fn find_matches<R: BufReadTrait>(
    reader: &mut R,
    pattern: &str,
    mut writter: impl std::io::Write,
) -> String {
    let mut line = String::new();
    let mut result = String::new();

    while reader.read_line(&mut line).unwrap() > 0 {
        if line.contains(&pattern) {
            writeln!(writter, "{}", line).unwrap();
            info!("Pattern found in line: {}", line);
            result.push_str(&line);
        }
        line.clear();
    }

    result
}

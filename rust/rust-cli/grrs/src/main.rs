use anyhow::{Context, Result};
use clap::Parser;
use log::{info, trace};
use spinners::{Spinner, Spinners};
use std::fs::File;
use std::io::{BufReader, Cursor};
use std::{thread::sleep, time::Duration};

fn main() -> Result<()> {
    let args = portogrrs::Cli::parse();
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

    portogrrs::find_matches(&mut reader, &args.pattern, &mut std::io::stdout());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_a_match() {
        let input = b"lorem";
        let mut reader = BufReader::new(Cursor::new(input));
        let mut writter = Vec::new();

        let result = portogrrs::find_matches(&mut reader, "lorem", &mut writter);

        assert_eq!(result, "lorem");
    }
}

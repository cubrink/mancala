use anyhow::Result;
use clap::Parser;

mod dispatch;
mod error;
mod perft;
mod types;

use crate::types::{PerftArgs, PerftResults};

fn main() -> Result<()> {
    // Check if no arguments were provided
    if std::env::args().len() == 1 {
        eprintln!("perft - A perft tool for Mancala position analysis");
        eprintln!();
        eprintln!("No arguments provided. Use --help for usage information.");
        std::process::exit(1);
    }
    let perft_args: PerftArgs = PerftArgs::parse();

    let results: PerftResults = dispatch::start(&perft_args)?;
    println!("{results}");

    Ok(())
}

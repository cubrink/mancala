use anyhow::Result;
use clap::Parser;

mod cli;
mod error;
mod perft;

use crate::cli::PerftArgs;
use crate::perft::start_perft;

fn main() -> Result<()> {
    // Check if no arguments were provided
    if std::env::args().len() == 1 {
        eprintln!("perft - A perft tool for Mancala position analysis");
        eprintln!();
        eprintln!("No arguments provided. Use --help for usage information.");
        std::process::exit(1);
    }
    let perft_args: PerftArgs = PerftArgs::parse();

    let results = start_perft(&perft_args)?;
    println!("Results:");
    println!("{results}");

    Ok(())
}

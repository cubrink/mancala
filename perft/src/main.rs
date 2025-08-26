use clap::Parser;
use mancala_lib::{GameState, Mancala};
use thiserror::Error;

/// Command line options from the user for the performance testing tool
#[derive(Parser, Debug)]
#[command(name = "perft")]
#[command(about = "A tool for Mancala position analysis")]
#[command(version)]
struct PerftArgs {
    /// Search depth
    depth: usize,

    /// Sequence of actions to reach starting position (comma-separated)
    /// Example: "1,3,5" applies actions 1, then 3, then 5
    #[arg(short, long)]
    actions: Option<String>,

    /// The amount of threads to use
    #[arg(short, long)]
    threads: Option<usize>,

    /// Show a count of states on a per-move basis
    divide: Option<bool>,
}

/// Internal options used to control a perft search
#[derive(Debug)]
struct PerftOptions {
    /// The depth to search to.
    depth: usize,
    /// The (optional) actions to take before searching.
    actions: Option<Vec<usize>>,
    /// The amount of threads to use, if None, single threaded.
    threads: Option<usize>,
    /// Whether to do a divide search.
    divide: bool,
}

#[derive(Debug, PartialEq, Error)]
enum PerftError {
    /// An unimplemented feature was requested
    #[error("The feature '{0}' has not been implemented yet.")]
    NotImplemented(String),
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check if no arguments were provided
    if std::env::args().len() == 1 {
        eprintln!("perft - A perft tool for Mancala position analysis");
        eprintln!();
        eprintln!("No arguments provided. Use --help for usage information.");
        std::process::exit(1);
    }
    let cli: PerftArgs = PerftArgs::parse();

    println!("Parsed CLI args:");
    println!("{cli:#?}");
    println!("Application logic for `perft` not yet implemented. Exiting early.");
    Ok(())
}

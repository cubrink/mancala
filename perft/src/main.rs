use clap::Parser;
use mancala_lib::{GameState, Mancala};

/// Command line optoins for the performance testing tool
#[derive(Parser, Debug)]
#[command(name = "perft")]
#[command(about = "A tool for Mancala position analysis")]
#[command(version)]
struct Cli {
    /// Search depth
    #[arg(short, long)]
    depth: Option<usize>,

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check if no arguments were provided
    if std::env::args().len() == 1 {
        eprintln!("perft - A perft tool for Mancala position analysis");
        eprintln!();
        eprintln!("No arguments provided. Use --help for usage information.");
        std::process::exit(1);
    }
    let cli: Cli = Cli::parse();

    println!("Parsed CLI args:");
    println!("{cli:#?}");
    println!("Application logic for `perft` not yet implemented. Exiting early.");
    Ok(())
}

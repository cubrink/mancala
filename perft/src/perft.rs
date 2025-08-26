use std::iter::Map;

use crate::cli::{PerftArgs, PerftOptions};
use anyhow::Result;
use mancala_lib::{GameState, Mancala};

#[derive(Debug)]
/// Packages the results of a perft run
pub struct PerftResults {
    /// The total number of nodes visited.
    total: usize,
    /// The options perft ran with
    options: PerftOptions,
    /// Information about the divide calculations, if relevant
    divide: Option<[usize; 12]>,
    /// The starting state that the search was based on.
    start: Mancala,
}

impl std::fmt::Display for PerftResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = todo!("Created formatted output");
        write!(f, "{}", output);
    }
}

pub fn perft_divide(game: &Mancala, options: &PerftOptions) -> [usize; 6] {
    todo!("Implement perft_divide");
}

pub fn perft(game: &Mancala, options: &PerftOptions) -> usize {
    todo!("Implement perft");
}

pub fn start_perft(args: &PerftArgs) -> Result<PerftResults> {
    todo!("Implement start_perft");
}

/// Prepares an initial gamestate given a perft options, or errors if the options are invalid.
///
/// * `options` - perft args
fn prepare_gamestate(options: &PerftOptions) -> anyhow::Result<Mancala> {
    match &options.actions {
        None => Ok(Mancala::default()),
        Some(actions) => {
            let mut game = Mancala::default();
            for action in actions {
                game.mut_act(*action)?;
            }
            Ok(game)
        }
    }
}

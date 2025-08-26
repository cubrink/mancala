use std::iter::Map;

use crate::cli::{PerftArgs, PerftOptions};
use anyhow::Result;
use mancala_lib::{GameState, Mancala};

#[derive(Debug)]
pub struct PerftResults {
    total: usize,
    args: PerftArgs,
    divide: Option<Map<usize, usize>>,
    start: Mancala,
}

impl std::fmt::Display for PerftResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = todo!("Created formatted output");
        write!(f, "{}", output);
    }
}

pub fn start_perft(args: &PerftArgs) -> Result<PerftResults> {
    todo!();
}

/// Prepares an initial gamestate given an perft args, or errors if the args are invalid.
///
/// * `options` - perft args
fn prepare_gamestate(args: &PerftArgs) -> anyhow::Result<Mancala> {
    let options = PerftOptions::try_from(args)?;
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

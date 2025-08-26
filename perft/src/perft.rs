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
        let output = format!("Total nodes: {}", self.total);
        write!(f, "{}", output)
    }
}

pub fn perft_divide(game: &Mancala, depth: usize) -> [usize; 6] {
    todo!("Implement perft_divide");
}

pub fn perft(game: &Mancala, depth: usize) -> usize {
    if depth == 0 || game.is_completed() {
        // If no depth to search, then we are just at this node
        return 1;
    } else if depth == 1 {
        // If the depth is just one, we look at valid actions from this node.
        return game.get_actions().len();
    } else {
        game.get_actions()
            .iter()
            .map(|a| perft(&game.act(*a).unwrap(), depth - 1))
            .sum()
    }
}

pub fn start_perft(args: &PerftArgs) -> Result<PerftResults> {
    let options = PerftOptions::try_from(args)?;
    let game = prepare_gamestate(&options)?;

    if options.divide {
        let divide_counts: [usize; 6] = match &options.threads {
            Some(threads) => todo!("Threading not implemented"),
            None => perft_divide(&game, options.depth),
        };
        let total: usize = divide_counts.iter().sum();
        let offset: usize = game.get_player() as usize * 7;
        let divide: Option<[usize; 12]> = Some(std::array::from_fn(|i| {
            if i < 6 {
                i + 1 + offset
            } else {
                divide_counts[i - 6]
            }
        }));
        let start = game;
        Ok(PerftResults {
            total,
            options,
            divide,
            start,
        })
    } else {
        let total: usize = match &options.threads {
            Some(threads) => todo!("Threading not implemented"),
            None => perft(&game, options.depth),
        };
        let divide = None;
        let start = game;
        Ok(PerftResults {
            total,
            options,
            divide,
            start,
        })
    }
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

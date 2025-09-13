use anyhow::Result;
use mancala_lib::{GameState, Mancala};
use perft::PerftError;
use perft::{PerftArgs, PerftOptions, PerftResults};
use perft::{perft, perft_divide, perft_divide_parallel, perft_parallel};

pub fn start(args: &PerftArgs) -> Result<PerftResults> {
    let options = PerftOptions::try_from(args)?;
    let game = prepare_gamestate(&options)?;

    if options.divide {
        let divide_counts: [usize; 6] = match &options.threads {
            Some(threads) => perft_divide_parallel(&game, options.depth, *threads)?,
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
            Some(threads) => perft_parallel(&game, options.depth, *threads)?,
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
                if game.get_actions().contains(action) {
                    game.mut_act(*action)?;
                } else {
                    Err(PerftError::InvalidStart(actions.clone()))?;
                }
            }
            Ok(game)
        }
    }
}

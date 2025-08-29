use crate::cli::{PerftArgs, PerftOptions};
use anyhow::Result;
use mancala_lib::{GameState, Mancala};
use rayon::prelude::*;
use rayon::ThreadPool;

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
        let mut lines: Vec<String> = Vec::new();
        let actions: String = match &self.options.actions {
            None => "".to_string(),
            Some(actions) => {
                let actions = actions
                    .iter()
                    .map(usize::to_string)
                    .collect::<Vec<String>>()
                    .join("->");
                format!(" [start->{}]", actions)
            }
        };
        lines.push(format!(
            "Perft({}) = {}{}",
            self.options.depth, self.total, actions
        ));
        match self.divide {
            None => (),
            Some(divide) => {
                let _: Vec<()> = divide[6..]
                    .iter()
                    .enumerate()
                    .map(|(idx, total)| {
                        let pit = idx + 1;
                        lines.push(format!("    [{pit}]: {total}"));
                    })
                    .collect();
            }
        }
        if self.options.ascii {
            lines.push("\n".to_string());
            lines.push(self.start.to_string());
        }
        let output = lines.join("\n");
        write!(f, "{output}")
    }
}

pub fn perft_divide(game: &Mancala, depth: usize) -> [usize; 6] {
    let actions = game.get_actions();
    let mut divide: [usize; 6] = [0; 6];
    let _: Vec<()> = actions
        .iter()
        .map(|a| {
            let game = game.act(*a).unwrap();
            let total = perft(&game, depth - 1);
            divide[(*a - 1) % 7] = total;
        })
        .collect();
    divide
}

pub fn perft(game: &Mancala, depth: usize) -> usize {
    if depth == 0 || game.is_completed() {
        // If no depth to search, then we are just at this node
        1
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

pub fn perft_parallel(game: &Mancala, depth: usize, pool: ThreadPool) -> usize {
    if depth == 0 || game.is_completed() {
        // If no depth to search, then we are just at this node
        1
    } else {
        pool.install(|| {
            game.get_actions()
                .par_iter()
                .map(|a| perft(&game.act(*a).unwrap(), depth - 1))
                .sum()
        })
    }
}

pub fn perft_divide_parallel(game: &Mancala, depth: usize, pool: ThreadPool) -> [usize; 6] {
    let actions = game.get_actions();
    let mut divide: [usize; 6] = [0; 6];
    let results: Vec<(usize, usize)> = pool.install(|| {
        actions
            .par_iter()
            .map(|a| {
                let game = game.act(*a).unwrap();
                let total = perft(&game, depth - 1);
                ((*a - 1) % 7, total)
            })
            .collect()
    });
    for (idx, total) in results.iter() {
        divide[*idx] = *total;
    }
    divide
}

pub fn start_perft(args: &PerftArgs) -> Result<PerftResults> {
    let options = PerftOptions::try_from(args)?;
    let game = prepare_gamestate(&options)?;

    if options.divide {
        let divide_counts: [usize; 6] = match &options.threads {
            Some(threads) => {
                let pool: rayon::ThreadPool = rayon::ThreadPoolBuilder::new()
                    .num_threads(*threads)
                    .build()?;
                perft_divide_parallel(&game, options.depth, pool)
            }
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
            Some(threads) => {
                let pool: rayon::ThreadPool = rayon::ThreadPoolBuilder::new()
                    .num_threads(*threads)
                    .build()?;
                perft_parallel(&game, options.depth, pool)
            }
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

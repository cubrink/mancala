use anyhow::Result;
use mancala_lib::{GameState, Mancala};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

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

#[cfg(feature = "parallel")]
pub fn perft_parallel(game: &Mancala, depth: usize, threads: usize) -> Result<usize> {
    if depth == 0 || game.is_completed() {
        // If no depth to search, then we are just at this node
        Ok(1)
    } else {
        Ok(get_pool(threads)?.install(|| {
            game.get_actions()
                .par_iter()
                .map(|a| perft(&game.act(*a).unwrap(), depth - 1))
                .sum()
        }))
    }
}

#[cfg(feature = "parallel")]
pub fn perft_divide_parallel(game: &Mancala, depth: usize, threads: usize) -> Result<[usize; 6]> {
    let actions = game.get_actions();
    let mut divide: [usize; 6] = [0; 6];
    let results: Vec<(usize, usize)> = get_pool(threads)?.install(|| {
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
    Ok(divide)
}

#[cfg(not(feature = "parallel"))]
pub fn perft_divide_parallel(
    _game: &Mancala,
    _depth: usize,
    _threads: usize,
) -> Result<[usize; 6]> {
    use crate::error::PerftError;
    Err(PerftError::MissingFeatures("parallel".to_string()))?
}

#[cfg(not(feature = "parallel"))]
pub fn perft_parallel(_game: &Mancala, _depth: usize, _threads: usize) -> Result<usize> {
    use crate::error::PerftError;
    Err(PerftError::MissingFeatures("parallel".to_string()))?
}

#[cfg(feature = "parallel")]
fn get_pool(threads: usize) -> Result<rayon::ThreadPool> {
    Ok(rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()?)
}

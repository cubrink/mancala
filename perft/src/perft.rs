use anyhow::Result;
use mancala_lib::{GameState, Mancala};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

pub fn perft_divide(game: &Mancala, depth: usize) -> [usize; 6] {
    let actions = game.get_actions();
    let mut divide: [usize; 6] = [0; 6];
    if depth == 0 {
        return divide;
    }
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

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    // The shortest games are of length 10
    #[rstest]
    #[case::perft_0("0", "1")]
    #[case::perft_1("1", "6")]
    #[case::perft_2("2", "35")]
    #[case::perft_3("3", "184")]
    #[case::perft_4("4", "918")]
    #[case::perft_5("5", "4405")]
    #[case::perft_6("6", "20830")]
    #[case::perft_7("7", "97014")]
    #[case::perft_8("8", "447866")]
    #[case::perft_9("9", "2049412")]
    #[case::perft_10("10", "9326089")]
    fn test_perft(#[case] depth: usize, #[case] gt: usize) {
        let count = perft(&Mancala::default(), depth);
        assert_eq!(count, gt);
    }

    #[rstest]
    #[case::perft_0("0", [0, 0, 0, 0, 0, 0])]
    #[case::perft_1("1", [1, 1, 1, 1, 1, 1])]
    #[case::perft_2("2", [6, 6, 5, 6, 6, 6])]
    #[case::perft_3("3", [32, 32, 30, 30, 30, 30])]
    #[case::perft_4("4", [161, 167, 140, 155, 150, 145])]
    #[case::perft_5("5", [761, 786, 696, 751, 719, 692])]
    fn test_perft_divide(#[case] depth: usize, #[case] gt: [usize; 6]) {
        let count = perft_divide(&Mancala::default(), depth);
        assert_eq!(count, gt);
    }
}

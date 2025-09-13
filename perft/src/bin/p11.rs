use mancala_lib::{GameState, Mancala};

#[inline(always)]
pub fn perft(game: &Mancala, depth: usize) -> usize {
    if game.is_completed() {
        return 1;
    }

    match depth {
        0 => 1,
        1 => game.get_actions().len(),
        _ => game
            .get_actions()
            .iter()
            .map(|&a| unsafe { perft(&game.act(a).unwrap_unchecked(), depth - 1) })
            .sum(),
    }
    // if depth == 0 || game.is_completed() {
    // // If no depth to search, then we are just at this node
    // 1
    // } else if depth == 1 {
    // // If the depth is just one, we look at valid actions from this node.
    // return game.get_actions().len();
    // } else {
    // game.get_actions()
    //     .iter()
    //     .map(|a| perft(&game.act(*a).unwrap(), depth - 1))
    //     .sum()
    // }
}

fn main() {
    perft(&Mancala::default(), 11);
}

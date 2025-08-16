mod error;
mod gamestate;
mod mancala;

pub use gamestate::GameState;
// const PITS: usize = 7;
// const ROWS: usize = 2;
// const N: usize = PITS * ROWS;

// struct GameState {
//     board: [usize; N],
//     player: usize,
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}

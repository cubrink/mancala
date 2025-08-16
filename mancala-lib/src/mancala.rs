use crate::error::MancalaError;
use crate::GameState;

const ROWS: usize = 2;
const PITS: usize = 7;

const N: usize = ROWS * PITS;

#[derive(Clone, Copy)]
struct Mancala {
    board: [usize; N],
    player: usize,
}

impl GameState for Mancala {
    type Error = MancalaError;

    fn act(&self, pit: usize) -> Result<Self, Self::Error> {
        todo!();
    }

    fn mut_act(&mut self, pit: usize) -> Result<Self, Self::Error> {
        todo!();
    }

    fn get_actions(&self) -> Vec<usize> {
        todo!();
    }

    fn get_player(&self) -> usize {
        todo!();
    }

    fn get_board(&self) -> usize {
        todo!();
    }

    fn is_completed(&self) -> bool {
        todo!();
    }

    fn get_winner(&self) -> Result<usize, Self::Error> {
        todo!();
    }

    fn at(&self, pit: usize) -> Result<Self, Self::Error> {
        todo!();
    }

    fn pop(&self, pit: usize) -> Result<(usize, Self), Self::Error> {
        todo!();
    }

    fn mut_pop(&self, pit: usize) -> Result<(usize, Self), Self::Error> {
        todo!();
    }
    fn is_scoring_pit(&self, pit: usize) -> Result<bool, Self::Error> {
        todo!();
    }
    fn get_opposite_pit(&self, pit: usize) -> Result<bool, Self::Error> {
        todo!();
    }
}

impl std::fmt::Display for Mancala {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::default::Default for Mancala {
    fn default() -> Self {
        todo!();
    }
}

impl std::convert::From<[usize; N]> for Mancala {
    fn from(value: [usize; N]) -> Self {
        todo!();
    }
}

impl std::convert::From<(usize, [usize; N])> for Mancala {
    fn from(value: (usize, [usize; N])) -> Self {
        todo!();
    }
}

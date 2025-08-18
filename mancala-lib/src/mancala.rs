use crate::error::MancalaError;
use crate::GameState;

const PITS: usize = 7;
const ROWS: usize = 2;
const N: usize = ROWS * PITS;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Mancala {
    board: [usize; N],
    player: bool,
}

impl Mancala {
    const _ADJACENT_TABLE: [Option<usize>; Self::N] = [
        None,
        Some(13),
        Some(12),
        Some(11),
        Some(10),
        Some(9),
        Some(8),
        None,
        Some(6),
        Some(5),
        Some(4),
        Some(3),
        Some(2),
        Some(1),
    ];

    fn new(board: [usize; N], player: bool) -> Self {
        Self { board, player }
    }

    fn check_bounds(pit: usize) -> Result<(), <Self as GameState>::Error> {
        if pit >= Self::N {
            Err(MancalaError::PitOutOfBounds(pit))?
        }
        Ok(())
    }
}

impl GameState for Mancala {
    type Error = MancalaError;
    type Player = bool;
    type Board = [usize; N];

    fn act(&self, pit: usize) -> Result<Self, Self::Error> {
        let mut game = self.clone();
        game.mut_act(pit)
    }

    fn mut_act(&mut self, _pit: usize) -> Result<Self, Self::Error> {
        todo!();
    }

    fn get_actions(&self) -> Vec<usize> {
        let player: usize = self.player as usize;
        let start: usize = 1 + PITS * player;
        let end: usize = PITS * (1 + player);
        self.board[start..end]
            .iter()
            .enumerate()
            .flat_map(|(idx, stones)| match stones {
                0 => None,
                _ => Some(idx + start),
            })
            .collect()
    }

    fn get_player(&self) -> bool {
        self.player
    }

    fn get_board(&self) -> &[usize; N] {
        &self.board
    }

    fn is_completed(&self) -> bool {
        for idx in 0..Self::N {
            if idx == 0 || idx == PITS {
                continue;
            }
            if self.board[idx] > 0 {
                return false;
            }
        }
        true
    }

    fn get_winner(&self) -> Result<Self::Player, Self::Error> {
        if !self.is_completed() {
            Err(MancalaError::GameNotEnded)?;
        }
        Ok(self.board[PITS] > self.board[0])
    }

    fn at(&self, pit: usize) -> Result<usize, Self::Error> {
        Self::check_bounds(pit)?;
        Ok(self.board[pit])
    }

    fn pop(&self, pit: usize) -> Result<(usize, Self), Self::Error> {
        let mut game = self.clone();
        game.mut_pop(pit)
    }

    fn mut_pop(&mut self, pit: usize) -> Result<(usize, Self), Self::Error> {
        Self::check_bounds(pit)?;
        if pit == 0 || pit == PITS {
            Err(MancalaError::NotPlayerPit(pit))?;
        }
        let stones = self.board[pit];
        self.board[pit] = 0;
        Ok((stones, *self))
    }

    fn is_scoring_pit(&self, pit: usize) -> Result<bool, Self::Error> {
        Self::check_bounds(pit)?;
        if pit == 0 || pit == (Self::N - 1) {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn get_opposite_pit(pit: usize) -> Result<usize, Self::Error> {
        Self::check_bounds(pit)?;
        match Self::_ADJACENT_TABLE[pit] {
            Some(p) => Ok(p),
            None => Err(MancalaError::NotPlayerPit(pit)),
        }
    }
}

impl std::fmt::Display for Mancala {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::default::Default for Mancala {
    fn default() -> Self {
        let board: [usize; N] = [0, 4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 4, 4, 4];
        let player: bool = false;
        Self { board, player }
    }
}

impl std::convert::From<[usize; N]> for Mancala {
    fn from(board: [usize; N]) -> Self {
        let player: bool = false;
        Self { board, player }
    }
}

impl std::convert::From<([usize; N], bool)> for Mancala {
    fn from(value: ([usize; N], bool)) -> Self {
        let (board, player) = value;
        Self { board, player }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_check_bounds_ok() {
        let result = Mancala::check_bounds(0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_bounds_below_n_ok() {
        let pit = Mancala::N - 1;
        let result = Mancala::check_bounds(pit);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_bounds_at_n_fails() {
        let pit = Mancala::N;
        let result = Mancala::check_bounds(pit);
        assert!(result.is_err(), "Bounds should be out of bounds");
        assert_eq!(result.unwrap_err(), MancalaError::PitOutOfBounds(pit));
    }

    #[test]
    fn test_check_bounds_past_n_fails() {
        let pit = Mancala::N + 1;
        let result = Mancala::check_bounds(pit);
        assert!(result.is_err(), "Bounds should be out of bounds");
        assert_eq!(result.unwrap_err(), MancalaError::PitOutOfBounds(pit));
    }

    #[rstest]
    fn test_get_opposite_oob_pit_fails(#[values(Mancala::N, Mancala::N+1)] pit: usize) {
        let opposite = Mancala::get_opposite_pit(pit);
        assert!(opposite.is_err(), "Bounds should be out of bounds");
        let error = opposite.unwrap_err();
        assert_eq!(error, MancalaError::PitOutOfBounds(pit));
    }

    #[rstest]
    fn test_get_opposite_scoring_pit_fails(#[values(0, 7)] pit: usize) {
        let opposite = Mancala::get_opposite_pit(pit);
        assert!(opposite.is_err(), "Scoring pits should not have opposites");
        let error = opposite.unwrap_err();
        assert_eq!(error, MancalaError::NotPlayerPit(pit));
    }

    #[rstest]
    #[case(1, 13)]
    #[case(2, 12)]
    #[case(3, 11)]
    #[case(4, 10)]
    #[case(5, 9)]
    #[case(6, 8)]
    #[case(8, 6)]
    #[case(9, 5)]
    #[case(10, 4)]
    #[case(11, 3)]
    #[case(12, 2)]
    #[case(13, 1)]
    fn test_get_opposite_scoring_pit_ok(#[case] pit: usize, #[case] gt: usize) {
        let opposite = Mancala::get_opposite_pit(pit);
        assert!(opposite.is_ok(), "Player pits should have opposites");
        let opposite = opposite.unwrap();
        assert_eq!(opposite, gt);
    }

    #[test]
    fn test_new_ok() {
        let board: [usize; N] = [0; N];
        let player: bool = false;
        let game = Mancala::new(board, player);
        assert_eq!(&board, game.get_board());
        assert_eq!(player, game.get_player());
    }

    #[test]
    fn test_default_ok() {
        let board: [usize; N] = [0, 4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 4, 4, 4];
        let player: bool = false;
        let gt = Mancala { board, player };
        let default = Mancala::default();
        assert_eq!(default, gt);
    }

    #[test]
    fn test_from_array_ok() {
        let board: [usize; N] = [5, 1, 2, 2, 4, 4, 4, 0, 4, 4, 4, 4, 4, 4];
        let player: bool = false;
        let gt = Mancala { board, player };
        let game = Mancala::from(board);
        assert_eq!(game, gt);
    }

    #[test]
    fn test_from_tuple_ok() {
        let board: [usize; N] = [5, 1, 2, 2, 4, 4, 4, 0, 4, 4, 4, 4, 4, 4];
        let player: bool = true;
        let gt = Mancala { board, player };
        let game = Mancala::from((board, player));
        assert_eq!(game, gt);
    }

    #[test]
    fn test_is_completed_true_ok() {
        let board: [usize; N] = [24, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0];
        let game = Mancala::from(board);
        let gt: bool = true;
        assert_eq!(game.is_completed(), gt);
    }

    #[test]
    fn test_is_completed_false_ok() {
        let board: [usize; N] = [5, 1, 2, 2, 4, 4, 4, 0, 4, 4, 4, 4, 4, 4];
        let game = Mancala::from(board);
        let gt: bool = false;
        assert_eq!(game.is_completed(), gt);
    }

    #[test]
    fn test_mut_pop_ok() {
        let mut game = Mancala::default();
        let (stones, game) = game.mut_pop(1).unwrap();
        assert_eq!(stones, 4);
        assert_eq!(game.at(1).unwrap(), 0);
    }

    #[test]
    fn test_mut_pop_oob_fails() {
        let mut game = Mancala::default();
        let error = game.mut_pop(Mancala::N).unwrap_err();
        assert_eq!(error, MancalaError::PitOutOfBounds(Mancala::N));
    }

    #[test]
    fn test_mut_pop_scoring_pit_fails() {
        let mut game = Mancala::default();
        let error = game.mut_pop(0).unwrap_err();
        assert_eq!(error, MancalaError::NotPlayerPit(0));
    }

    #[test]
    fn test_get_actions_player_1() {
        let board: [usize; N] = [0, 1, 2, 3, 0, 0, 0, 7, 0, 4, 0, 5, 0, 6];
        let player: bool = false;
        let game = Mancala::from((board, player));
        let gt: Vec<usize> = vec![1, 2, 3];
        let actions = game.get_actions();
        assert_eq!(actions, gt);
    }

    #[test]
    fn test_get_actions_player_2() {
        let board: [usize; N] = [0, 1, 2, 3, 0, 0, 0, 7, 0, 4, 0, 5, 0, 6];
        let player: bool = true;
        let game = Mancala::from((board, player));
        let gt: Vec<usize> = vec![9, 11, 13];
        let actions = game.get_actions();
        assert_eq!(actions, gt);
    }
}

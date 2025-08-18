use crate::error::MancalaError;
use crate::GameState;

const PITS: usize = 7;
const ROWS: usize = 2;
const N: usize = ROWS * PITS;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Mancala {
    board: [usize; N],
    player: usize,
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

    fn new(board: [usize; N], player: usize) -> Result<Self, MancalaError> {
        if player > 1 {
            Err(MancalaError::InvalidPlayer(player))?;
        }
        Ok(Self { board, player })
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

    fn act(&self, _pit: usize) -> Result<Self, Self::Error> {
        todo!();
    }

    fn mut_act(&mut self, _pit: usize) -> Result<Self, Self::Error> {
        todo!();
    }

    fn get_actions(&self) -> Vec<usize> {
        todo!();
    }

    fn get_player(&self) -> usize {
        self.player
    }

    fn get_board(&self) -> &[usize; N] {
        &self.board
    }

    fn is_completed(&self) -> bool {
        todo!();
    }

    fn get_winner(&self) -> Result<usize, Self::Error> {
        todo!();
    }

    fn at(&self, _pit: usize) -> Result<Self, Self::Error> {
        todo!();
    }

    fn pop(&self, _pit: usize) -> Result<(usize, Self), Self::Error> {
        todo!();
    }

    fn mut_pop(&self, _pit: usize) -> Result<(usize, Self), Self::Error> {
        todo!();
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
        let player: usize = 0;
        Self { board, player }
    }
}

impl std::convert::From<[usize; N]> for Mancala {
    fn from(_value: [usize; N]) -> Self {
        todo!();
    }
}

impl std::convert::From<(usize, [usize; N])> for Mancala {
    fn from(_value: (usize, [usize; N])) -> Self {
        todo!();
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
        let player = 0;
        let game = Mancala::new(board, player);
        assert!(game.is_ok());
        let game = game.unwrap();
        assert_eq!(&board, game.get_board());
        assert_eq!(player, game.get_player());
    }

    #[test]
    fn test_new_wrong_player_fails() {
        let board: [usize; N] = [0; N];
        let player = 2;
        let game = Mancala::new(board, player);
        assert!(game.is_err());
        assert_eq!(game.unwrap_err(), MancalaError::InvalidPlayer(player));
    }

    #[test]
    fn test_default_ok() {
        let board: [usize; N] = [0, 4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 4, 4, 4];
        let player: usize = 0;
        let gt = Mancala { board, player };
        let default = Mancala::default();
        assert_eq!(default, gt);
    }
}

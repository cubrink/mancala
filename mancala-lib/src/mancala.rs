use crate::error::MancalaError;
use crate::GameState;

const PITS: usize = 7;
const ROWS: usize = 2;
const N: usize = ROWS * PITS;

/// Holds data to represent a game of mancala
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mancala {
    /// The mancala board represented as a flat array.
    ///
    /// Indices 0 and 7 (N/2), represent the scoring pits
    /// At index 0 is the scoring pit of player 1
    /// At index 7 is the scoring pit of player 2
    board: [usize; N],

    /// A flag to keep track of whose turn it is in the game
    player: bool,
}

/// Holds data about what regions the player can make moves for.
#[derive(Debug)]
struct MoveData {
    /// The index of the first non-scoring pit for a player.
    pub start: usize,

    /// The index of the last non-scoring pit for a player (inclusive)
    pub end: usize,

    /// The index of the player's scoring pit
    pub scorepit: usize,
}

/// Holds data from computing Mancala::walk
#[derive(Debug, PartialEq)]
struct WalkData {
    /// A board saying which indicies were visited.
    pub visited: [usize; N],
    /// A note saying the last index to be visited.
    pub final_idx: usize,
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

    fn check_bounds(pit: usize) -> Result<(), <Self as GameState>::Error> {
        if pit >= Self::N {
            Err(MancalaError::PitOutOfBounds(pit))?
        }
        Ok(())
    }

    /// Returns an iterator of each index that gets visited when moving
    /// a pile of stones.
    ///
    /// * `start` - The location to start walking from
    /// * `steps` - How many steps to take
    fn walk(start: usize, steps: usize) -> WalkData {
        let player = start / Self::PITS;
        let skip = player * Self::PITS;

        let wrap_count = steps / (N - 1);
        let mut visited: [usize; N] = [wrap_count; N];
        visited[skip] = 0;
        let steps = steps % (N - 1);

        let mut offset: usize = 0;
        for s in 1..=steps {
            if (start + s + offset) % Self::N == skip {
                offset += 1;
            }
            let idx = (start + s + offset) % Self::N;
            visited[idx] += 1;
        }
        let final_idx = (start + steps + offset) % Self::N;
        WalkData { visited, final_idx }
    }
}

impl GameState for Mancala {
    type Error = MancalaError;
    type Player = bool;
    type Board = [usize; N];
    type Action = usize;
    type ActionSequence = heapless::Vec<Self::Action, { PITS - 1 }>;

    fn act(&self, pit: usize) -> Result<Self, Self::Error> {
        let mut game = *self;
        game.mut_act(pit)
    }

    fn mut_act(&mut self, pit: usize) -> Result<Self, Self::Error> {
        // Pre-compute useful quantities
        let stones = self.mut_pop(pit)?;
        let a = MoveData {
            start: 1,
            end: Self::PITS - 1,
            scorepit: Self::PITS,
        };
        let b = MoveData {
            start: Self::PITS + 1,
            end: Self::N - 1,
            scorepit: 0,
        };
        let (a, b) = match self.player {
            false => (a, b),
            true => (b, a),
        };

        // General game loop
        let WalkData { visited, final_idx } = Self::walk(pit, stones);
        visited
            .iter()
            .enumerate()
            // .filter_map(|(idx, visited)| match visited {
            //     0 => None,
            //     _ => Some(idx),
            // })
            .for_each(|(idx, v)| self.board[idx] += v);

        // Handle special rules
        // 1. Capture player pit if landed in your own empty pit
        if self.board[final_idx] == 1 {
            match Self::_ADJACENT_TABLE[final_idx] {
                None => (),
                Some(opposite) => match self.board[opposite] {
                    0 => {}
                    x => {
                        self.board[opposite] = 0;
                        self.board[final_idx] = 0;
                        self.board[a.scorepit] += x + 1;
                    }
                },
            };
        }
        // 2. Finish game if player has no moves left
        if self.board[a.start..=a.end].iter().sum::<usize>() == 0 {
            self.board[b.scorepit] += self.board[b.start..=b.end].iter().sum::<usize>();
            self.board[b.start..=b.end].fill(0);
        }
        // 3. Go again if landed in the scoring pit
        if final_idx != a.scorepit {
            self.player = !self.player;
        }
        Ok(*self)
    }

    fn get_actions(&self) -> Self::ActionSequence {
        let player: usize = self.player as usize;
        let start: usize = 1 + PITS * player;
        let end: usize = PITS * (1 + player);
        let actions: Self::ActionSequence = self.board[start..end]
            .iter()
            .enumerate()
            .flat_map(|(idx, stones)| match stones {
                0 => None,
                _ => Some(idx + start),
            })
            .collect();
        actions
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
        let mut game = *self;
        let stones = game.mut_pop(pit)?;
        Ok((stones, game))
    }

    fn mut_pop(&mut self, pit: usize) -> Result<usize, Self::Error> {
        Self::check_bounds(pit)?;
        if pit == 0 || pit == PITS {
            Err(MancalaError::NotPlayablePit(pit))?;
        }
        let stones = self.board[pit];
        self.board[pit] = 0;
        Ok(stones)
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
            None => Err(MancalaError::NotPlayablePit(pit)),
        }
    }
}

impl std::fmt::Display for Mancala {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let border = "_____________________________________________";
        let output = [
            border.to_string(),
            format!(
                "|      | {0:2} | {1:2} | {2:2} || {3:2} | {4:2} | {5:2} |      |\\",
                self.board[1],
                self.board[2],
                self.board[3],
                self.board[4],
                self.board[5],
                self.board[6],
            ),
            format!(
                "|  {0:2}  |--------------||--------------|  {1:>2}  | |",
                self.board[0], self.board[7],
            ),
            format!(
                "|      | {0:2} | {1:2} | {2:2} || {3:2} | {4:2} | {5:2} |      | |",
                self.board[13],
                self.board[12],
                self.board[11],
                self.board[10],
                self.board[9],
                self.board[8],
            ),
            format!("{border}| |"),
            " \\_____________________\\\\_____________________\\|".to_string(),
        ]
        .join("\n");

        write!(f, "{output}")
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
        assert_eq!(error, MancalaError::NotPlayablePit(pit));
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
        let stones = game.mut_pop(1).unwrap();
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
        assert_eq!(error, MancalaError::NotPlayablePit(0));
    }

    #[test]
    fn test_get_actions_player_1() {
        let board: [usize; N] = [0, 1, 2, 3, 0, 0, 0, 7, 0, 4, 0, 5, 0, 6];
        let player: bool = false;
        let game = Mancala::from((board, player));
        // let gt: <Mancala as GameState>::ActionSequence = vec![1, 2, 3];
        let gt = <Mancala as GameState>::ActionSequence::from([1, 2, 3]);
        let actions = game.get_actions();
        assert_eq!(actions, gt);
    }

    #[test]
    fn test_get_actions_player_2() {
        let board: [usize; N] = [0, 1, 2, 3, 0, 0, 0, 7, 0, 4, 0, 5, 0, 6];
        let player: bool = true;
        let game = Mancala::from((board, player));
        // let gt: <Mancala as GameState>::ActionSequence = vec![9, 11, 13];
        let gt = <Mancala as GameState>::ActionSequence::from([9, 11, 13]);
        let actions = game.get_actions();
        assert_eq!(actions, gt);
    }

    #[test]
    fn test_to_string_ok() {
        let mut game = Mancala::default();
        game.board[0] = 10;
        game.board[1] = 20;
        game.board[9] = 55;
        let output: String = game.to_string();
        println!();
        println!("{output}");
    }

    #[test]
    fn test_walk_simple_ok() {
        let visited = Mancala::walk(1, 4);
        let gt = WalkData {
            visited: [0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            final_idx: 5,
        };
        assert_eq!(visited, gt);
    }

    #[test]
    fn test_walk_scorepit_ok() {
        let visited = Mancala::walk(5, 5);
        let gt = WalkData {
            visited: [0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0],
            final_idx: 10,
        };
        assert_eq!(visited, gt);
    }

    #[test]
    fn test_walk_p2_scorepit_ok() {
        let visited = Mancala::walk(12, 5);
        let gt = WalkData {
            visited: [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            final_idx: 3,
        };
        assert_eq!(visited, gt);
    }

    #[test]
    fn test_walk_skip_p2_scorepit_ok() {
        let visited = Mancala::walk(6, 8);
        let gt = WalkData {
            visited: [0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1],
            final_idx: 1,
        };
        assert_eq!(visited, gt);
    }

    #[test]
    fn test_walk_skip_p1_scorepit_ok() {
        let visited = Mancala::walk(13, 8);
        let gt = WalkData {
            visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0],
            final_idx: 8,
        };
        assert_eq!(visited, gt);
    }

    #[test]
    fn test_walk_wrap_twice_ok() {
        let visited = Mancala::walk(1, 26);
        let gt = WalkData {
            visited: [0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
            final_idx: 1,
        };
        assert_eq!(visited, gt);
    }

    #[test]
    fn test_move_simple_ok() {
        let mut game = Mancala::from([0; N]);
        let gt = Mancala::from(([0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0], true));
        game.board[1] = 4;
        game = game.mut_act(1).unwrap();
        assert_eq!(game, gt);
    }

    #[test]
    fn test_move_one_wrap_ok() {
        let mut game = Mancala::from([0; N]);
        let gt = Mancala::from(([0, 0, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 0], true));
        game.board[1] = 13;
        game = game.mut_act(1).unwrap();
        assert_eq!(game, gt);
    }

    #[test]
    fn test_move_two_wrap_ok() {
        let mut game = Mancala::from([0; N]);
        let gt = Mancala::from(([100, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2], true));
        game.board[0] = 100;
        game.board[1] = 26;
        game = game.mut_act(1).unwrap();
        assert_eq!(game, gt);
    }

    #[test]
    fn test_capture_player_one_ok() {
        //                                      0, 1, 2, 3, 4, 5, 6, 7, 8, 9,10,11,12,13
        let mut game = Mancala::from(([0, 1, 0, 0, 0, 0, 1, 2, 0, 0, 0, 0, 1, 1], false));
        let ground_t = Mancala::from(([0, 0, 0, 0, 0, 0, 1, 4, 0, 0, 0, 0, 0, 1], true));
        game = game.mut_act(1).unwrap();
        assert_eq!(game, ground_t);
    }

    #[test]
    fn test_capture_player_two_collect_player_one_ok() {
        //                                      0, 1, 2, 3, 4, 5, 6, 7, 8, 9,10,11,12,13
        let mut game = Mancala::from(([0, 1, 0, 0, 0, 0, 1, 2, 0, 0, 0, 0, 1, 0], true));
        let ground_t = Mancala::from(([2, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0], false));
        game = game.mut_act(12).unwrap();
        assert_eq!(game, ground_t);
    }

    // Test that
    // Happy path:
    // Capture mechanic works for both players
    //
    // Fail cases:
    // Move only own pieces
    // Can't move score pits
    //
}

use arrayvec::ArrayVec;
use std::fmt;

const ROWS: usize = 2;
const PITS: usize = 7;
const N: usize = ROWS * PITS;
type Stone = usize;
type Board = [Stone; ROWS * PITS];
type Pit = usize;
type Player = usize;
type ActionList = ArrayVec<Pit, { PITS - 1 }>;
type PitList = ArrayVec<Pit, { ROWS * PITS - 1 }>;

#[derive(Debug)]
pub enum GameError {
    Unknown,
    IndexOutOfBounds(u8),
    NoStone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct State {
    player: Player,
    board: Board,
}

impl From<[Pit; N]> for State {
    fn from(value: [Pit; N]) -> Self {
        Self {
            player: 0,
            board: value,
        }
    }
}

impl From<(usize, [Pit; N])> for State {
    fn from(value: (usize, [Pit; N])) -> Self {
        let (player, board) = value;
        Self { player, board }
    }
}

impl State {
    pub fn new() -> Self {
        let player: Player = 0;
        let board: Board = [0, 4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 4, 4, 4];
        Self::from((player, board))
    }

    pub fn get_player(&self) -> Player {
        self.player
    }

    pub fn view(&self) -> Board {
        self.board
    }

    pub fn moves(&self) -> ActionList {
        todo!("Only iterate over appropriate half, make convenience func?");
        let a: usize = self.player * PITS;
        let b: usize = a + PITS;
        self.board[a..b]
            .iter()
            .enumerate()
            .filter_map(
                |(i, stones)| {
                    if i % PITS == 0 {
                        None
                    } else {
                        Some(i as Pit)
                    }
                },
            )
            .collect()
    }

    pub fn act(&self, action: Pit) -> Result<State, String> {
        if action % PITS == 0 {
            Err("Cannot act on scoring pit!".to_string())?;
        }
        let mut state = *self;
        let mut stones: Stone = state.pop(action);

        let skip_idx = ((state.player * PITS) + PITS) % N;
        let mut idx = (action + 1) % N;

        while stones > 0 {
            if idx == skip_idx {
                idx = (idx + 1) % N;
            }
            state.board[idx] += 1;
            stones -= 1;
            idx = (idx + 1) % N;
        }
        Ok(state)
    }

    fn pop(&mut self, action: Pit) -> Stone {
        if let Some(element) = self.board.get_mut(action) {
            let stones = *element;
            *element = 0;
            stones
        } else {
            0
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.board)
    }
}
// Todo, impl Display

fn divmod(n: Stone, d: Pit) -> (Stone, Stone) {
    (n / (d as Stone), n % (d as Stone))
}

fn delta(stones: Stone, origin: usize, skip: usize) -> Board {
    let (a, b) = divmod(stones, N - 1);
    let mut board: Board = [a; N];
    board[skip] -= a;
    let mut idx = (origin + 1) % N;
    for k in 0..b {
        println!("{}", k);
        // Increment index
        idx = if idx == skip {
            (idx + 2) % N
        } else {
            (idx + 1) % N
        };
        board[idx] += 1
    }
    board
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_move_pit_0_fails() {
        let game = State::from([4; N]);
        let game = game.act(0);
        let gt = Err("Cannot act on scoring pit!".to_string());
        assert_eq!(game, gt);
    }

    #[test]
    fn test_move_pit_1_ok() {
        let game = State::from([4; N]);
        let game = game.act(1).unwrap();
        let gt = State::from([4, 0, 5, 5, 5, 5, 4, 4, 4, 4, 4, 4, 4, 4]);
        assert_eq!(game, gt);
    }
}

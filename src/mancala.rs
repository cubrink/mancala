use arrayvec::ArrayVec;
use std::fmt;

type Stones = u8;
type Board = [[Stones; State::MAX]; 2];
type Action = usize;
type Player = usize;
type ActionList = ArrayVec<Action, 6>;

pub enum GameError {
    Unknown,
    IndexOutOfBounds(u8),
    NoStones,
}

#[derive(Debug, Clone, Copy)]
pub struct State {
    current_player: Player,
    other_player: Player,
    board: Board,
}

impl State {
    const MAX: usize = 8;
    const MIN: usize = 0;

    pub fn new() -> Self {
        let current_player = 0;
        let other_player = 1;
        let board: [[u8; 8]; 2] = [
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
        ];
        Self {
            current_player,
            other_player,
            board,
        }
    }

    pub fn player(&self) -> Player {
        self.current_player
    }

    pub fn view(&self) -> Board {
        self.board.clone()
    }

    pub fn moves(&self) -> ActionList {
        self.board[self.current_player]
            .iter()
            .enumerate()
            .filter_map(|(i, stones)| {
                if (i == State::MIN || i == (State::MAX - 1)) {
                    None
                } else {
                    Some(i as Action)
                }
            })
            .collect()
    }

    pub fn act(&self, action: Action) -> State {
        let mut state = self.clone();
        let stones: Stones = state.pop(action);

        for i in 1..=stones {
            let (_a, _b) = divmod(i.into(), 13);
        }

        todo!();
    }

    fn pop(&mut self, action: Action) -> Stones {
        let stones = self.board[self.current_player][action];
        self.board[self.current_player][action] = 0;
        stones
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.board)
    }
}
// Todo, impl Display

fn divmod(n: usize, d: usize) -> (usize, usize) {
    (n / d, n % d)
}

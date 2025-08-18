use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum MancalaError {
    /// A pit provided was not in bounds of the board
    #[error("A pit '{0}' was used that was out of bounds of the entire board.")]
    PitOutOfBounds(usize),

    /// The game has not ended yet
    #[error("The game has not ended yet")]
    GameNotEnded,

    /// The pit is not a playable pit
    #[error("The pit {0} is not a playable pit")]
    NotPlayerPit(usize),
}

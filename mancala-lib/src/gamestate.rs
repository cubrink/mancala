pub const PITS: usize = 7;
pub const ROWS: usize = 2;
pub const N: usize = PITS * ROWS;

pub trait GameState
where
    Self: From<Self::Board>,
    Self: From<(Self::Board, Self::Player)>,
    Self: std::fmt::Display,
    Self: std::default::Default,
    Self: Clone,
{
    type Error: std::error::Error;
    type Player;
    type Board;

    const PITS: usize = PITS;
    const ROWS: usize = ROWS;
    const N: usize = N;

    /// Performs an action on the game state.
    ///
    /// Immutably acts on the game state with a given action.
    /// The resulting game state is returned if the action was valid.
    /// Otherwise, an error is returned.
    ///
    /// * `pit` - Action to perform on the game state.
    ///
    fn act(&self, pit: usize) -> Result<Self, Self::Error>;

    /// Performs an action on the game state.
    ///
    /// Mutably acts on the game state with a given action.
    /// The resulting game state is returned if the action was valid.
    /// Otherwise, an error is returned.
    ///
    /// * `pit` - Action to perform on the game state.
    ///
    fn mut_act(&mut self, pit: usize) -> Result<Self, Self::Error>;

    /// Returns a vector of legal actions.
    fn get_actions(&self) -> Vec<usize>;

    /// Returns the current player in the game.
    fn get_player(&self) -> Self::Player;

    /// Returns a view of the current board.
    fn get_board(&self) -> &Self::Board;

    /// Returns if a game is completed.
    fn is_completed(&self) -> bool;

    /// Returns the player who won.
    fn get_winner(&self) -> Result<Self::Player, Self::Error>;

    /// Returns the number of stones at a certain pit.
    ///
    /// * `pit` - Pit to look at
    fn at(&self, pit: usize) -> Result<usize, Self::Error>;

    /// Removes the number of stones at a certain pit
    ///
    /// Returns a tuple (stones, state) if the action is valid.
    /// Otherwise, returns an error
    ///
    /// * `pit` - Pit to remove stones from
    fn pop(&self, pit: usize) -> Result<(usize, Self), Self::Error>;

    /// Mutably removes the number of stones at a certain pit
    ///
    /// Returns a tuple (stones, state) if the action is valid.
    /// Otherwise, returns an error
    ///
    /// * `pit` - Pit to remove stones from
    fn mut_pop(&mut self, pit: usize) -> Result<(usize, Self), Self::Error>;

    /// Returns if a pit is a scoring pit.
    ///
    /// If the pit is not a valid pit, an error is returned.
    ///
    /// * `pit` - Pit to check if it a scoring pit
    fn is_scoring_pit(&self, pit: usize) -> Result<bool, Self::Error>;

    /// Returns the pit that is opposite of the selected pit
    ///
    /// If the pit does not have an opposite (invalid or scoring),
    /// an error is raised
    ///
    /// * `pit` - Pit to get opposite pit of
    fn get_opposite_pit(pit: usize) -> Result<usize, Self::Error>;
}

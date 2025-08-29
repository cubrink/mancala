use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum PerftError {
    /// An action provided is invalid
    #[error("The action '{0}' is not a valid action.")]
    InvalidAction(String),
}

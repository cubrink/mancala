use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum PerftError {
    /// An unimplemented feature was requested
    #[error("The feature '{0}' has not been implemented yet.")]
    NotImplemented(String),
    /// An action provided is invalid
    #[error("The action '{0}' is not a valid action.")]
    InvalidAction(String),
}

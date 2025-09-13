mod algorithms;
mod error;
mod types;

pub use algorithms::{perft, perft_divide, perft_divide_parallel, perft_parallel};
pub use error::PerftError;
pub use types::{PerftArgs, PerftOptions, PerftResults};

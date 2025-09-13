mod error;
mod perft;
mod types;

pub use error::PerftError;
pub use perft::{perft, perft_divide, perft_divide_parallel, perft_parallel};
pub use types::{PerftArgs, PerftOptions, PerftResults};

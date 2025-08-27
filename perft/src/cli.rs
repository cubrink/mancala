use crate::error::PerftError;
use clap::Parser;

/// Command line options from the user for the performance testing tool
#[derive(Parser, Debug)]
#[command(name = "perft")]
#[command(about = "A tool for Mancala position analysis")]
#[command(version)]
pub struct PerftArgs {
    /// Search depth
    depth: usize,

    /// Sequence of actions to reach starting position (comma-separated)
    /// Example: "1,3,5" applies actions 1, then 3, then 5
    #[arg(short, long)]
    actions: Option<String>,

    /// The amount of threads to use
    #[arg(short, long)]
    threads: Option<usize>,

    /// Show a count of states on a per-move basis
    #[arg(long, action = clap::ArgAction::SetTrue)]
    divide: Option<bool>,

    /// Show ascii art of board at output
    #[arg(long, action = clap::ArgAction::SetTrue)]
    ascii: Option<bool>,
}

/// Internal options used to control a perft search
#[derive(Debug)]
pub struct PerftOptions {
    /// The depth to search to.
    pub depth: usize,
    /// The (optional) actions to take before searching.
    pub actions: Option<Vec<usize>>,
    /// The amount of threads to use, if None, single threaded.
    pub threads: Option<usize>,
    /// Whether to do a divide search.
    pub divide: bool,
    /// Show ascii art of board at output
    pub ascii: bool,
}

impl TryFrom<&PerftArgs> for PerftOptions {
    type Error = PerftError;

    /// Validates Perft Arguments after they've been parsed.
    ///
    /// Arguments:
    /// * `args` - The parsed arguments from `clap`
    ///
    /// Errors:
    /// * `PerftError::NotImplemented`
    ///   Raised if an unimplemented flag is requested
    fn try_from(args: &PerftArgs) -> Result<Self, Self::Error> {
        let actions = match &args.actions {
            None => None,
            Some(a) => {
                let actions = a
                    .split(',')
                    .map(|segment| segment.trim().parse::<usize>())
                    .collect::<Vec<Result<usize, _>>>();
                if actions.iter().any(Result::is_err) {
                    Err(PerftError::InvalidAction(a.to_string()))?;
                }
                Some(
                    actions
                        .iter()
                        .flatten()
                        .map(usize::clone)
                        .collect::<Vec<usize>>(),
                )
            }
        };
        let threads: Option<usize> = args.threads;
        let depth: usize = args.depth;
        let divide: bool = args.divide.unwrap_or(false);
        let ascii: bool = args.ascii.unwrap_or(false);
        Ok(PerftOptions {
            depth,
            actions,
            threads,
            divide,
            ascii,
        })
    }
}

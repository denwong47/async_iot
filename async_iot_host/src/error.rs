use thiserror::Error;

use tide;

/// Various Error types that can arise from Graphaurus operations.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum AppError {
    #[error("A lock for {0} is poisoned; execution cannot continue.")]
    LockPoisoned(&'static str),

    #[error("HTTP host encountered an error: {0:?}")]
    TideError(tide::Error),

    #[error("unknown error occurred: {context}")]
    Unknown { context: String },
}

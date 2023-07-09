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

    #[error("Cannot listen to Ctrl-C calls: {message}")]
    CtrlCError { message: String },

    #[error(
        "Attempted to log to a path at {path:?}, but it was not initialised in this `AppState`."
    )]
    AppPathNotRecognised { path: String },

    #[error("A remote host requested a termination with error: {message}")]
    RemoteRequestedTermination { message: String },

    #[error("unknown error occurred: {context}")]
    Unknown { context: String },
}

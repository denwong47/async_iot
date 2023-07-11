use thiserror::Error;

/// Various Error types that can arise from Graphaurus operations.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum VendorsError {
    #[error("A lock for {0} is poisoned; execution cannot continue.")]
    LockPoisoned(&'static str),

    #[error("Cannot listen to Ctrl-C calls: {message}")]
    CtrlCError { message: String },

    #[error("unknown error occurred: {context}")]
    Unknown { context: String },
}

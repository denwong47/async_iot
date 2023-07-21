use thiserror::Error;

#[cfg(feature = "python")]
use pyo3::{PyErr, PyTypeInfo};

/// Various Error types that can arise from Graphaurus operations.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum LocalError {
    #[error("A lock for {0} is poisoned; execution cannot continue.")]
    LockPoisoned(&'static str),

    #[error("Could not complete a HTTP request: {context}")]
    HTTPRequestFailed { context: String },

    #[error("Could not parse a datetime from text: {text}")]
    DateTimeParsingError { text: String },

    #[error("The URL requested is invalid: {context}")]
    InvalidURL { context: String },

    #[error("unknown error occurred: {context}")]
    Unknown { context: String },
}

impl LocalError {
    /// Consume this [`LocalError`], wrap it in a [`PyErr`], then return it.
    #[cfg(feature = "python")]
    pub fn into_pyerr<E>(self) -> PyErr
    where
        E: PyTypeInfo,
    {
        PyErr::new::<E, _>(self.to_string())
    }
}

impl From<reqwest::Error> for LocalError {
    fn from(value: reqwest::Error) -> Self {
        Self::HTTPRequestFailed {
            context: value.to_string(),
        }
    }
}

impl From<url::ParseError> for LocalError {
    fn from(value: url::ParseError) -> Self {
        Self::InvalidURL {
            context: value.to_string(),
        }
    }
}

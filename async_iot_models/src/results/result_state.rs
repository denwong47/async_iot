use std::error::Error;

use serde::{ser::SerializeMap, Serialize};

use super::ExtendedResult;
/// Result of getting a system state item.
///
/// This class is for [`Serialize`] only.
#[derive(Clone, Debug)]
pub enum ResultState {
    Ok,
    WithWarnings(Vec<String>),
    Err(String),
}

impl<T, E> From<&Result<T, E>> for ResultState
where
    E: Error,
{
    fn from(value: &Result<T, E>) -> Self {
        match value {
            Ok(_) => Self::Ok,
            Err(err) => Self::Err(err.to_string()),
        }
    }
}

impl<T, E> From<&ExtendedResult<T, E>> for ResultState
where
    E: Error,
{
    fn from(value: &ExtendedResult<T, E>) -> Self {
        match value {
            ExtendedResult::Ok(_) => Self::Ok,
            ExtendedResult::WithWarnings(_, warnings) => Self::WithWarnings(warnings.clone()),
            ExtendedResult::Err(err) => Self::Err(err.to_string()),
        }
    }
}

impl Serialize for ResultState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Ok => {
                let mut state = serializer.serialize_map(Some(1))?;
                state.serialize_entry("status", "ok")?;
                state.end()
            }
            Self::WithWarnings(warnings) => {
                let mut state = serializer.serialize_map(Some(2))?;
                state.serialize_entry("status", "ok")?;
                state.serialize_entry("warnings", warnings)?;
                state.end()
            }
            Self::Err(msg) => {
                let mut state = serializer.serialize_map(Some(2))?;
                state.serialize_entry("status", "error")?;
                state.serialize_entry("error", &msg)?;
                state.end()
            }
        }
    }
}

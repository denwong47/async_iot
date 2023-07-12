use std::error::Error;

use serde::Serialize;
use serde_json::{self, Value as JsonValue};

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
            ExtendedResult::Err(_, err) => Self::Err(err.to_string()),
        }
    }
}

impl From<&ResultState> for JsonValue {
    fn from(value: &ResultState) -> Self {
        match value {
            ResultState::Ok => serde_json::json!({
                "_status": "ok",
            }),
            ResultState::WithWarnings(warnings) => serde_json::json!({
                "_status": "ok",
                "_warnings": warnings,
            }),
            ResultState::Err(msg) => serde_json::json!({
                "_status": "error",
                "_error": &msg,
            }),
        }
    }
}

impl From<&ResultState> for serde_json::Map<String, JsonValue> {
    fn from(value: &ResultState) -> Self {
        if let JsonValue::Object(map) = value.into() {
            map
        } else {
            panic!(
                "`ResultState` did not produce a `serde_json::Value::Object(map)` - please check `impl From<&ResultState> for JsonValue` implementation."
            )
        }
    }
}

impl Serialize for ResultState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        JsonValue::from(self).serialize(serializer)
    }
}

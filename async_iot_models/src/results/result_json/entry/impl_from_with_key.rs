use serde::Serialize;
use serde_json::{self, Value as JsonValue};

use crate::results::{ExtendedResult, ResultState};
use crate::traits::FromWithKey;

use super::ResultJsonEntry;

impl<T> FromWithKey<T> for ResultJsonEntry
where
    T: Serialize,
{
    fn from_with_key(key: &str, value: T) -> Self {
        let value = serde_json::to_value(value);

        match value {
            Ok(JsonValue::Null) => Self::new_scalar::<()>(key.to_string(), ResultState::Ok, None),
            Ok(JsonValue::Object(map)) => Self::new_mapping(key.to_string(), ResultState::Ok)
                .with_children(
                    map.into_iter()
                        .map(|(key, value)| Self::from_with_key(&key, value))
                        .collect(),
                ),
            Ok(value) => Self::new_scalar(key.to_string(), ResultState::Ok, Some(value)),
            Err(err) => Self::from_err(key, err),
        }
    }
}

impl<T, E> FromWithKey<ExtendedResult<T, E>> for ResultJsonEntry
where
    Self: FromWithKey<T>,
    E: ToString,
{
    fn from_with_key(key: &str, value: ExtendedResult<T, E>) -> Self {
        match value {
            ExtendedResult::Ok(inner_value) => FromWithKey::<T>::from_with_key(key, inner_value),
            ExtendedResult::WithWarnings(inner_value, warnings) => {
                let entry: Self = FromWithKey::<T>::from_with_key(key, inner_value);
                entry.with_state(ResultState::WithWarnings(warnings))
            }
            ExtendedResult::Err(_exit_code, err) => Self::from_err(key, err),
        }
    }
}

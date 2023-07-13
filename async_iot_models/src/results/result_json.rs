use std::error::Error;

use serde::Serialize;
use serde_json::{self, Value as JsonValue};
use time;

use super::ResultState;

use crate::{config, logger, traits::ResultToOption};

#[derive(Clone, Debug)]
pub struct ResultJsonEntry {
    pub key: String,
    pub state: ResultState,
    pub value: Option<JsonValue>,
    pub children: Option<Vec<Self>>,
}

impl ResultJsonEntry {
    /// Recursively add the values of itself and children to the the provided
    /// `_results` and `map`, to create a [`ResultJson`] instance.
    pub(crate) fn add_to(
        &self,
        _results: &mut serde_json::Map<String, JsonValue>,
        map: &mut serde_json::Map<String, JsonValue>,
    ) {
        let mut _result_entry = serde_json::Map::from(&self.state);
        let mut _map_entry = self
            .children
            .as_ref()
            .map(|children| {
                JsonValue::Object(children.iter().fold(
                    if let Some(value) = self.value.as_ref() {
                        let mut map = serde_json::Map::new();
                        map.insert("_value".to_owned(), value.clone());
                        map
                    } else {
                        serde_json::Map::new()
                    },
                    |mut map, entry| {
                        entry.add_to(&mut _result_entry, &mut map);
                        map
                    },
                ))
            })
            .unwrap_or(serde_json::to_value(&self.value).unwrap_or(JsonValue::Null));

        _results.insert(self.key.clone(), JsonValue::Object(_result_entry));
        map.insert(self.key.clone(), _map_entry);
    }

    /// Instantiate a new instance of [`ResultJsonEntry`] with a scalar value.
    pub fn new_scalar<T>(key: String, state: ResultState, value: Option<T>) -> Self
    where
        serde_json::Value: From<T>,
    {
        Self {
            key,
            state,
            value: value.map(serde_json::Value::from),
            children: None,
        }
    }

    /// Instantiate a new instance of [`ResultJsonEntry`] with a scalar value.
    pub fn new_mapping(key: String, state: ResultState) -> Self {
        Self {
            key,
            state,
            value: None,
            children: Some(Vec::new()),
        }
    }

    /// Chained method for adding any number of children to this [`ResultJsonEntry`].
    pub fn with_children(mut self, mut children: Vec<Self>) -> Self {
        if self.children.is_none() {
            logger::warning(
                &format!(
                    "Key {key} is a scalar value, but `with_children` is called on it. The existing value will be moved to a subkey of `_value`.",
                    key=&self.key
                )
            );
            self.children = Some(Vec::new());
        };

        self.children
            .as_mut()
            .map(|container| container.append(&mut children));
        self
    }

    /// Chained method for adding a child entry of the same type.
    pub fn add_child_entry(self, entry: Self) -> Self {
        self.with_children(vec![entry])
    }

    /// Chained method for adding a child to this [`ResultJsonEntry`].
    pub fn add_scalar_child(
        self,
        key: String,
        state: ResultState,
        value: Option<JsonValue>,
    ) -> Self {
        self.with_children(vec![Self::new_scalar(key, state, value)])
    }

    /// Create a new instance of [`ResultJson`] from a [`JsonValue`].
    pub fn from_value(key: &str, value: JsonValue) -> Self {
        match value {
            JsonValue::Null => Self::new_scalar::<()>(key.to_string(), ResultState::Ok, None),
            JsonValue::Object(map) => Self::new_mapping(key.to_string(), ResultState::Ok)
                .with_children(
                    map.into_iter()
                        .map(|(key, value)| Self::from_value(&key, value))
                        .collect(),
                ),
            value => Self::new_scalar(key.to_string(), ResultState::Ok, Some(value)),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ResultJson {
    results: Vec<ResultJsonEntry>,
}

impl ResultJson {
    /// Create a new empty [`ResultJson`] instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new empty [`ResultJson`] instance, with allocation limited to a certain
    /// capacity,
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            results: Vec::with_capacity(capacity),
        }
    }

    /// Return a [`ResultJson`] down with only the included keys.
    ///
    /// # Note
    ///
    /// Any keys not present in the instance are ignored.
    pub fn get(&self, keys: &[&str]) -> Self {
        Self {
            results: self
                .results
                .iter()
                .filter(|entry| keys.contains(&entry.key.as_str()))
                .map(|result| result.clone())
                .collect(),
        }
    }

    /// Append a result to this [`ResultJson`] in place.
    ///
    /// This function is assured to succeed, yet it is possible that the resultant
    /// JSON will contain the error messsage if serialization failed.
    pub fn append_result<S, T>(
        &mut self,
        key: &S,
        state: ResultState,
        value: T,
        children: Option<Vec<ResultJsonEntry>>,
    ) where
        S: ToString,
        T: Serialize,
    {
        let try_value = serde_json::to_value(value);

        match try_value {
            Ok(value) => self.results.push(ResultJsonEntry {
                key: key.to_string(),
                state,
                value: Some(value),
                children,
            }),
            Err(err) => self.results.push(ResultJsonEntry {
                key: key.to_string(),
                state: ResultState::Err(format!(
                    "Value cannot be JSON serialized due to '{}'. Original state: {}",
                    err,
                    serde_json::to_string(&state)
                        .unwrap_or("(Cannot serialize `ResultState`.)".to_owned())
                )),
                value: None,
                children,
            }),
        }
    }

    /// Add a result to a [`ResultJson`], and return the resultant instance.
    ///
    /// This function is assured to succeed, yet it is possible that the resultant
    /// JSON will contain the error messsage if serialization failed.
    pub fn add_result<S, T>(
        mut self,
        key: &S,
        state: ResultState,
        value: T,
        children: Option<Vec<ResultJsonEntry>>,
    ) -> Self
    where
        S: ToString,
        T: Serialize,
    {
        self.append_result(key, state, value, children);
        self
    }
}

impl ResultJson {
    /// Create a new instance of [`ResultJson`] from an Error by populating it into all
    /// the keys supplied.
    pub fn from_err<E>(value: E, keys: &[&str]) -> Self
    where
        E: Error,
    {
        keys.iter().fold(Self::new(), |json, key| {
            json.add_result(
                key,
                ResultState::Err(value.to_string()),
                Option::<()>::None,
                None,
            )
        })
    }
}

impl Serialize for ResultJson {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let _timestamp = time::OffsetDateTime::now_utc()
            .format(&config::DATETIME_FORMAT)
            .to_option();

        let (_results, mut map) = self.results.iter().fold(
            Ok((
                serde_json::Map::with_capacity(self.results.len() + 2),
                serde_json::Map::with_capacity(self.results.len() + 2),
            )),
            |result_chain, entry| {
                result_chain.and_then(|(mut _results, mut map)| {
                    entry.add_to(&mut _results, &mut map);

                    Ok((_results, map))
                })
            },
        )?;

        map.insert(
            "_timestamp".to_owned(),
            serde_json::to_value(&_timestamp).unwrap_or(JsonValue::Null),
        );
        map.insert("_results".to_owned(), JsonValue::Object(_results));

        map.serialize(serializer)
    }
}

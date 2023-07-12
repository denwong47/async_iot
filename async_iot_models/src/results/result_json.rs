use std::{collections::HashMap, error::Error};

use serde::{ser::SerializeMap, Serialize};
use serde_json::{self, Value as JsonValue};
use time;

use super::ResultState;

use crate::{config, traits::ResultToOption};

#[derive(Clone, Debug, Default)]
pub struct ResultJson {
    results: Vec<(String, ResultState, Option<JsonValue>)>,
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
                .filter(|(key, ..)| keys.contains(&key.as_str()))
                .map(|result| result.clone())
                .collect(),
        }
    }

    /// Append a result to this [`ResultJson`] in place.
    ///
    /// This function is assured to succeed, yet it is possible that the resultant
    /// JSON will contain the error messsage if serialization failed.
    pub fn append_result<S, T>(&mut self, key: &S, state: ResultState, value: T)
    where
        S: ToString,
        T: Serialize,
    {
        let try_value = serde_json::to_value(value);

        match try_value {
            Ok(value) => self.results.push((key.to_string(), state, Some(value))),
            Err(err) => self.results.push((
                key.to_string(),
                ResultState::Err(format!(
                    "Value cannot be JSON serialized due to '{}'. Original state: {}",
                    err,
                    serde_json::to_string(&state)
                        .unwrap_or("(Cannot serialize `ResultState`.)".to_owned())
                )),
                None,
            )),
        }
    }

    /// Add a result to a [`ResultJson`], and return the resultant instance.
    ///
    /// This function is assured to succeed, yet it is possible that the resultant
    /// JSON will contain the error messsage if serialization failed.
    pub fn add_result<S, T>(mut self, key: &S, state: ResultState, value: T) -> Self
    where
        S: ToString,
        T: Serialize,
    {
        self.append_result(key, state, value);
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
            json.add_result(key, ResultState::Err(value.to_string()), Option::<()>::None)
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
                HashMap::new(),
                serializer.serialize_map(Some(self.results.len() + 2))?,
            )),
            |result_chain, (key, state, value)| {
                result_chain.and_then(|(mut _results, mut map)| {
                    _results.insert(key.to_string(), state);

                    match map.serialize_entry(&key.to_string(), value) {
                        Ok(_) => Ok((_results, map)),
                        Err(e) => Err(e),
                    }
                })
            },
        )?;

        map.serialize_entry("_timestamp", &_timestamp)?;
        map.serialize_entry("_results", &_results)?;

        map.end()
    }
}

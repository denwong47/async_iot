use std::error::Error;

use serde::Serialize;
use serde_json::{self, Value as JsonValue};
use time;

use super::entry::ResultJsonEntry;

use crate::{config, results::ResultState, traits::ResultToOption};

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

    /// Chained method for adding any number of children to this [`ResultJson`].
    pub fn with_children(mut self, mut children: Vec<ResultJsonEntry>) -> Self {
        self.results.append(&mut children);
        self
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
    pub fn from_err<E>(keys: &[&str], err: E) -> Self
    where
        E: Error,
    {
        keys.iter().fold(Self::new(), |json, key| {
            json.add_result(
                key,
                ResultState::Err(err.to_string()),
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

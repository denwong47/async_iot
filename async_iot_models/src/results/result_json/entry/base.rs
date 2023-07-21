use serde::Serialize;
use serde_json::{self, Value as JsonValue};

use crate::results::ResultState;

use crate::{logger, traits::FromWithKey};

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

    /// Return [`None`] if this [`ResultJsonEntry`] is scalar, otherwise
    /// return [`Option<usize>`] indicating the number of children this instance
    /// contains.
    pub fn children_count(&self) -> Option<usize> {
        self.children.as_ref().map(|children| children.len())
    }

    /// Check if this [`ResultJsonEntry`] contains a scalar value only.
    pub fn is_scalar(&self) -> bool {
        self.children_count().is_none()
    }

    /// Chained method for changing the state of this [`ResultJsonEntry`].
    pub fn with_state(mut self, state: ResultState) -> Self {
        self.state = state;

        self
    }

    /// Create a new instance of [`ResultJsonEntry`] from a result, which contains a
    /// serializable object.
    pub fn from_result<T, E>(key: &str, value: Result<T, E>) -> Self
    where
        Self: FromWithKey<T>,
        E: ToString,
    {
        match value {
            Ok(value) => Self::from_with_key(key, value),
            Err(err) => Self::from_err(key, err),
        }
    }

    /// Create a new instance of [`ResultJsonEntry`] from an Error by populating it into all
    /// the keys supplied.
    pub fn from_err<E>(key: &str, value: E) -> Self
    where
        E: ToString,
    {
        ResultJsonEntry::new_scalar(
            key.to_string(),
            ResultState::Err(value.to_string()),
            Option::<JsonValue>::None,
        )
    }

    /// Create a new instance of [`ResultJson`] from an instance that can [`Serialize`].
    pub fn from_serializable<T, E>(key: &str, value: &T) -> Self
    where
        T: Serialize,
        E: ToString,
    {
        match serde_json::to_value(value) {
            Ok(value) => Self::from_with_key(key, value),
            Err(err) => Self::from_err(key, err),
        }
    }
}

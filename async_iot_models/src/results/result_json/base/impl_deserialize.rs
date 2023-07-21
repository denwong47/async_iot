use serde::{de, Deserialize, Deserializer};
use serde_json::{Map as JsonMap, Value as JsonValue, self};

use time::{self, OffsetDateTime};

use crate::{config, results::{ResultState, ResultJsonEntry}};
use super::ResultJson;

struct InterimResultJson {
    pub _results: JsonMap<String, JsonValue>,
    pub _timestamp: OffsetDateTime,
    pub body: JsonValue,
}

impl InterimResultJson {
    fn try_from_value<'de, D>(mut value: JsonMap<String, JsonValue>) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Set up the verification functions for the macro below.
        let results_verify =
            |json_value| {
                if let JsonValue::Object(map) = json_value {
                    Ok(map)
                } else {
                    Err(de::Error::custom(
                        format!(
                            "JSON object deserialized, yet `_results` does not contain an object: {json_value:?}",
                        )
                    ))
                }
            }
        ;
        let timestamp_verify =
            |json_value| {
                if let JsonValue::String(iso_date) = json_value {
                    OffsetDateTime::parse(&iso_date, config::DATETIME_FORMAT)
                    .map_err(
                        |_| de::Error::custom(
                            format!(
                                "`_timestamp` value is not a valid ISO8601 date time: {iso_date}"
                            )
                        )
                    )
                } else {
                    Err(de::Error::custom(
                        format!(
                            "JSON object deserialized, yet `_timestamp` does not contain a string: {json_value:?}",
                        )
                    ))
                }
            }
        ;

        macro_rules! expand_fields {
            (
                $((
                    $field:ident,
                    $verify:path
                )),+$(,)?
            ) => {
                $(
                    let $field =
                        value
                        .remove(stringify!($field))
                        .ok_or(
                            de::Error::custom(
                                format!(
                                    "JSON object deserialized, yet `_results` key not found: {map:?}",
                                    map=&value
                                )
                            )
                        )
                        .and_then($verify)?
                    ;

                )*
            };
        }

        expand_fields!(
            (_results, results_verify),
            (_timestamp, timestamp_verify),
        );

        // The only thing remaining should be the actual values.
        Ok(Self {
            _results,
            _timestamp,
            body: value,
        })
    }

    /// Remove any states related to the current key in this [`InterimResultJson`], then
    /// return a the generated [`ResultState`] instance if successful.
    fn drain_state<'de, D>(&mut self) -> Result<ResultState, D::Error>
    where
        D: Deserializer<'de>
    {
        match (
            self._results.remove("_status"),
            self._results.remove("_warnings"),
            self._results.remove("_error")
        ) {
            (Some(_), Some(JsonValue::Array(warnings)), None) => {
                // We have some warnings.
                Ok(ResultState::WithWarnings(
                    warnings
                    .into_iter()
                    .map(|value| value.to_string())
                    .collect()
                ))
            },
            (Some(JsonValue::String(str)), None, None)
            if str.eq("ok") => {
                // We are all good.
                Ok(ResultState::Ok)
            },
            (Some(JsonValue::String(str)), None, Some(JsonValue::String(msg)))
            if str.eq("error") => {
                // Something bad happened
                Ok(ResultState::Err(msg.to_owned()))
            },
            _ => {
                Err(de::Error::custom(
                    format!(
                        "Results is neither a `Ok`, `WithWarnings` or `Error`: {results:?}",
                        results=self._results,
                    )
                ))
            }
        }
    }

    /// Consume this [`InterimResultJson`], strip away the current layer of
    pub fn explode<'de, D>(mut self) -> Result<
        (
            OffsetDateTime,
            ResultJsonEntry, // The current key
        ),
        D::Error
    >
    where
        D: Deserializer<'de>,
    {
        // After this,
        let state = self.drain_state::<D>()?;



    }
}

impl<'de> Deserialize<'de> for ResultJson {
    /// This is not a full deserialization of the a [`ResultJson`]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let deserialized = JsonValue::deserialize(deserializer)?;
        if let JsonValue::Object(map) = deserialized {
            let interim = InterimResultJson::try_from_value::<'de, D>(map)?;

            ResultJsonEntry::try_from_interim::<D>("", interim).map(Self::from)
        } else {
            // JSON deserialization succeeded, but its not an object.
            Err(de::Error::custom(
                format!(
                    "To deserialize into `ResultJson`, a JSON object is expected at the top level; yet found: {deserialized:?}"
                )
            ))
        }
    }
}

impl ResultJsonEntry {

    /// Try to create a [`ResultJson`] from an [`InterimResultJson`].
    #[doc(hidden)]
    fn try_from_interim<'de, D>(
        key: &str,
        mut interim: InterimResultJson,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let entries: Result<Vec<ResultJsonEntry>, D::Error> = Result::from_iter(
            interim
            .body
            .into_iter()
            .map(
                |(key, body)| {
                    // TODO make this a function
                    if let Some(JsonValue::Object(_results)) = interim._results.remove(&key) {
                        let (
                            timestamp,
                            mut entry,
                            children_opt
                        ) = interim.explode()?;

                        if let Some(children) = children_opt {
                            entry.with_children(
                                Result::from_iter(
                                    children
                                    .map(
                                        |interim| Self::try_from_interim(&entry.key, interim)
                                    )
                                )?
                            )
                        } else {

                        }
                    } else {
                        Err(de::Error::custom(
                            format!(
                                "JSON object deserialized, yet `_results` does not contain an object: {body:?}",
                            )
                        ))
                    }

                }
            )
        );

        Self::new_mapping(key, state)
    }
}

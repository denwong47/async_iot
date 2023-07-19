use serde::{de, Deserialize, Deserializer};
use serde_json::{Error as JsonError, Map as JsonMap, Value as JsonValue};
use serde_with::{serde_as, TimestampSeconds};
use std::marker::PhantomData;

use time::OffsetDateTime;

use super::ResultJson;

struct InterimResultJson {
    pub _results: JsonMap<String, JsonValue>,
    pub _timestamp: OffsetDateTime,
    pub body: JsonMap<String, JsonValue>,
}

impl InterimResultJson {
    fn try_from<'de, D>(mut value: JsonMap<String, JsonValue>) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let _results =
            value
            .remove("_results")
            .ok_or(
                de::Error::custom(
                    format!(
                        "JSON object deserialized, yet `_results` key not found: {map:?}",
                        map=&value
                    )
                )
            )
            .and_then(
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
            )?
        ;
        todo!()
    }
}

impl<'de> Deserialize<'de> for ResultJson {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let deserialized = JsonValue::deserialize(deserializer)?;
        if let JsonValue::Object(map) = deserialized {
            let interim = InterimResultJson::try_from::<'de, D>(map)?;

            todo!()
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

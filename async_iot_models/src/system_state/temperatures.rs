use std::io;

use serde_json::value::{Number, Value};
use systemstat::{self, Platform};

use super::SystemState;
use crate::results;

#[cfg(target_os = "linux")]
use serde_json::value::Map;

/// Get temperatures of this machine.
pub fn temperatures(key: &str, sys: &SystemState) -> results::ResultJsonEntry {
    results::ResultJsonEntry::new_mapping(key.to_owned(), results::ResultState::Ok).with_children(
        vec![
            results::ResultJsonEntry::from_result("cpu", cpu_temp(sys)),
            #[cfg(target_os = "linux")]
            results::ResultJsonEntry::from_result("sensors", sensors_temp(sys)),
            #[cfg(not(target_os = "linux"))]
            results::ResultJsonEntry::from_err(
                "sensors",
                io::Error::new(
                    io::ErrorKind::Unsupported,
                    "Sensor temperatures are only supported on Linux.".to_owned(),
                ),
            ),
        ],
    )
}

/// Internal function to fetch CPU temperature.
fn cpu_temp(sys: &SystemState) -> io::Result<Value> {
    sys.systemstat
        .cpu_temp()
        .map(|temp| Value::Number(Number::from_f64(temp as f64).unwrap_or(i64::MIN.into())))
}

/// Internal function to fetch sensors temperature.
#[cfg(target_os = "linux")]
fn sensors_temp(sys: &SystemState) -> io::Result<Value> {
    let mut map = Map::new();

    io::Result::<()>::from_iter(sys.psutil_sensors.iter().map(|item_result| {
        item_result
            .as_ref()
            .map(|item| {
                map.insert(
                    item.unit().to_string(),
                    Value::Number(
                        Number::from_f64(item.current().celsius()).unwrap_or(i64::MIN.into()),
                    ),
                );
            })
            .map_err(|err| io::Error::new(io::ErrorKind::Unsupported, err.to_string()))
    }))
    .map(|_| Value::Object(map))
}

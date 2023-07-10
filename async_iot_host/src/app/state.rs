use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, RwLock,
    },
};

use serde::{ser::SerializeMap, Serialize};
use time;

use async_iot_models::{logger, traits::ResultToOption};

use crate::config;
use crate::error::AppError;

/// A struct to contain information about a single visit.
#[cfg(feature = "detailed_logging")]
#[derive(Clone, Debug, Serialize)]
pub struct AppVisit {
    path: String,
    remote: Option<String>,
}

/// A struct holding statistics about app use.
pub struct AppState {
    pub start_time: time::OffsetDateTime,
    visit_counts: RwLock<HashMap<String, AtomicU64>>,

    #[cfg(feature = "detailed_logging")]
    latest_visits: RwLock<Vec<AppVisit>>,
}

impl AppState {
    /// Instantiate a new [`AppState`] with the timestamps set to the current
    /// UTC time.
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            start_time: time::OffsetDateTime::now_utc(),
            visit_counts: RwLock::new(HashMap::new()),

            #[cfg(feature = "detailed_logging")]
            latest_visits: RwLock::new(Vec::with_capacity(config::LOG_LATEST_VISITS)),
        })
    }

    /// Insert a new path into the visitor counter, together with an initial value.
    pub fn insert_path(&self, path: &str, count: u64) -> Result<(), AppError> {
        self.visit_counts
            .write()
            .map(|mut map| {
                map.insert(path.to_string(), count.into());
            })
            .map_err(|_| AppError::LockPoisoned("visits counter"))
    }

    /// All paths need to be initialised before visits can be logged.
    pub fn init_path(&self, path: &str) -> Result<(), AppError> {
        self.insert_path(path, 0)
    }

    /// Calculate the uptime of this app.
    pub fn uptime(&self) -> time::Duration {
        time::OffsetDateTime::now_utc() - self.start_time
    }

    /// Log a new visit.
    pub fn log_visit(&self, path: &str, remote: Option<&str>) -> Result<(), AppError> {
        match remote {
            Some(remote) => logger::info(&format!("Rendering '{path}' for '{remote}'.")),
            None => logger::info("Rendering '{path}' for unknown remote."),
        }

        self.visit_counts
            .read()
            .or(Err(AppError::LockPoisoned("visits counter")))
            .and_then(|map| {
                if let Some(counter) = map.get(path) {
                    Ok(counter.fetch_add(1, Ordering::Relaxed))
                } else {
                    // We HAVE to drop `map` out of scope here.
                    // `insert_path()` uses `visit_counts.write()`, so as long as `map`
                    // exists, `visit_counts.write()` will block.
                    drop(map);
                    self.insert_path(path, 1).and(Ok(0))
                }
            })
            .and_then(|_| {
                #[cfg(not(feature = "detailed_logging"))]
                {
                    Ok(())
                }

                #[cfg(feature = "detailed_logging")]
                {
                    let visit = AppVisit {
                        path: path.to_string(),
                        remote: remote.map(|s| s.to_string()),
                    };

                    self.latest_visits
                        .write()
                        .map_err(|_| AppError::LockPoisoned("Latest Visits"))
                        .and_then(|mut latest_visits| {
                            if latest_visits.len() >= latest_visits.capacity() {
                                latest_visits.remove(0);
                            }

                            latest_visits.push(visit);

                            Ok(())
                        })
                }
            })
    }
}

impl Serialize for AppState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(4))?;

        map.serialize_entry(
            "start_time",
            &self.start_time.format(&config::DATETIME_FORMAT).to_option(),
        )?;
        map.serialize_entry("uptime", &self.uptime().as_seconds_f64())?;
        map.serialize_entry("visit_counts", &self.visit_counts)?;

        #[cfg(feature = "detailed_logging")]
        map.serialize_entry(
            "latest_vists",
            &self
                .latest_visits
                .read()
                .unwrap() // TODO Address poisoning
                .iter()
                .collect::<Vec<_>>(),
        )?;

        map.end()
    }
}

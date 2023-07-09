use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};

use serde::{ser::SerializeMap, Serialize};
use time;

use async_iot_models::{logger, traits::ResultToOption};

use crate::config;
use crate::error::AppError;

#[cfg(feature = "detailed_logging")]
use std::sync::RwLock;

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
    visit_counts: HashMap<String, AtomicU64>,

    #[cfg(feature = "detailed_logging")]
    latest_visits: RwLock<Vec<AppVisit>>,
}

impl AppState {
    /// Instantiate a new [`AppState`] with the timestamps set to the current
    /// UTC time.
    pub fn new(paths: &[&str]) -> Arc<Self> {
        let mut instance = Self {
            start_time: time::OffsetDateTime::now_utc(),
            visit_counts: HashMap::new(),

            #[cfg(feature = "detailed_logging")]
            latest_visits: RwLock::new(Vec::with_capacity(config::LOG_LATEST_VISITS)),
        };

        paths.into_iter().for_each(|path| instance.init_path(path));

        Arc::new(instance)
    }

    /// All paths need to be initialised before visits can be logged.
    pub fn init_path(&mut self, path: &str) {
        self.visit_counts.insert(path.to_string(), 0.into());
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

        match self.visit_counts.get(path) {
            Some(counter) => Ok(counter.fetch_add(1, Ordering::Relaxed)),
            None => Err(AppError::AppPathNotRecognised {
                path: path.to_string(),
            }),
        }
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

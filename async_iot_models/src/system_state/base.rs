use async_trait::async_trait;
use std::sync::RwLock;

use serde::{Deserialize, Serialize};

use sysinfo::{self, SystemExt};
use systemstat::{self, Platform};

use crate::{
    results,
    traits::{DeserializeWith, HasCachedState, HasState},
    LocalError,
};

use super::{
    cpu::cpu, disks::disks, memory::memory, networks::networks, system::system,
    temperatures::temperatures,
};

#[cfg(target_os = "linux")]
use psutil::{self, sensors};

/// An empty struct that acts as a wrapper for associated functions.
#[derive(Serialize, Deserialize)]
pub struct SystemState {
    #[serde(skip_serializing)]
    #[serde(deserialize_with = "systemstat::System::deserialize_with")]
    pub systemstat: systemstat::System,

    #[serde(skip_serializing)]
    #[serde(deserialize_with = "sysinfo::System::deserialize_with")]
    pub sysinfo: sysinfo::System,

    #[serde(skip)]
    _cache: RwLock<Option<results::ResultJson>>,

    #[cfg(target_os = "linux")]
    #[serde(skip_serializing)]
    #[serde(
        deserialize_with = "Vec<psutil::Result<sensors::TemperatureSensor>>::deserialize_with"
    )]
    pub psutil_sensors: Vec<psutil::Result<sensors::TemperatureSensor>>,
}

impl SystemState {
    /// Create a [`SystemState`] with the relevant [`System`] instances.
    pub fn new() -> Self {
        Self {
            systemstat: systemstat::System::new(),
            sysinfo: sysinfo::System::new_all(),

            _cache: RwLock::new(None),

            #[cfg(target_os = "linux")]
            psutil_sensors: sensors::temperatures(),
        }
    }
}

macro_rules! expand_fields {
    (
        $((
            $field: ident,
            $func: expr
        )),*$(,)?
    ) => {
        #[async_trait]
        impl HasState for SystemState {
            /// Get a [`results::ResultJson`] with only the specified keys.
            async fn try_get(
                &self,
                keys: &[&str],
            ) -> Result<results::ResultJson, LocalError> {
                Ok(
                    results::ResultJson::new()
                    .with_children(
                        {
                            keys
                            .iter()
                            .map(
                                |key| match key {
                                    $(
                                        &stringify!($field) => $func(&stringify!($field), self),
                                    )*
                                    _ => results::ResultJsonEntry::from_err(
                                        key,
                                        format!("Requested key of {key} not recgonised.")
                                    )
                                }
                            )
                            .collect()
                        }
                    )
                )
            }

            /// Get a [`Vec`] of all the available keys for [`SystemState::get()`].
            fn available_keys() -> Vec<&'static str> {
                vec![
                    $(
                        stringify!($field),
                    )*
                ]
            }

            /// Get a [`results::ResultJson`] with all the available
            /// keys.
            async fn try_all(&self) -> Result<results::ResultJson, LocalError> {
                self.try_get(&Self::available_keys()).await
            }
        }
    }
}

expand_fields!(
    (system, system),
    (cpu, cpu),
    (temperatures, temperatures),
    (memory, memory),
    (disks, disks),
    (networks, networks),
);

#[async_trait]
impl HasCachedState for SystemState {
    async fn locked_cache<'a>(&'a self) -> &'a RwLock<Option<results::ResultJson>> {
        &self._cache
    }
}

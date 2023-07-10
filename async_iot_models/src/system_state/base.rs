use sysinfo::{self, SystemExt};
use systemstat::{self, Platform};

use crate::{results, traits::ResultToOption};

use super::{cpu::cpu, networks::networks, system::system, temperatures::temperatures};

#[cfg(target_os = "linux")]
use psutil::{self, sensors};

/// An empty struct that acts as a wrapper for associated functions.
pub struct SystemState {
    pub systemstat: systemstat::System,
    pub sysinfo: sysinfo::System,

    #[cfg(target_os = "linux")]
    pub psutil_sensors: Vec<psutil::Result<sensors::TemperatureSensor>>,
}

macro_rules! expand_fields {
    (
        $((
            $field: ident,
            $func: expr
        )),*$(,)?
    ) => {
        impl SystemState {
            /// Create a [`SystemState`] with the relevant [`System`] instances.
            pub fn new() -> Self {
                Self {
                    systemstat: systemstat::System::new(),
                    sysinfo: sysinfo::System::new_all(),

                    #[cfg(target_os = "linux")]
                    psutil_sensors: sensors::temperatures(),
                }
            }

            /// Get a [`results::ResultJson`] with only the specified keys.
            pub fn get(
                &self,
                keys: &[&str],
            ) -> results::ResultJson {
                let mut json = results::ResultJson::new();

                $(
                    if keys.contains(&stringify!($field)) {
                        let result = $func(self);
                        json.append_result(
                            &stringify!($field),
                            results::ResultState::from(&result),
                            result.to_option(),
                        );
                    }
                )*

                json
            }

            /// Get a [`results::ResultJson`] with all the available
            /// keys.
            pub fn all(&self) -> results::ResultJson {
                self.get(
                    &[
                        $(
                            stringify!($field),
                        )*
                    ]
                )
            }
        }
    }
}

expand_fields!(
    (system, system),
    (cpu, cpu),
    (temperatures, temperatures),
    (networks, networks),
);

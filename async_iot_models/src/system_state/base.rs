use std::collections::HashMap;

use systemstat::{self, Platform};

use serde::Serialize;

use crate::{results, traits::ResultToOption};

use super::{cpu::cpu_load, networks::networks, temperatures::temperatures, InterfaceState};

#[derive(Clone, Debug, Serialize)]
pub struct SystemState {
    _results: HashMap<&'static str, results::ResultState>,
    cpu_load: Option<systemstat::CPULoad>,
    temperatures: Option<HashMap<&'static str, f32>>,
    networks: Option<HashMap<String, Option<InterfaceState>>>,
}

impl Default for SystemState {
    fn default() -> Self {
        let mut _results = HashMap::new();

        let sys = systemstat::System::new();

        macro_rules! expand_fields {
            (
                $((
                    $field: ident,
                    $func: expr
                )),*$(,)?
            ) => {
                $(
                    let $field = $func(&sys);
                    _results.insert(stringify!($field), results::ResultState::from(&$field));

                )*
            };
        }

        expand_fields!(
            (cpu_load, cpu_load),
            (temperatures, temperatures),
            (networks, networks),
        );

        // Creates the instance.
        Self {
            _results,
            cpu_load: cpu_load.to_option(),
            temperatures: temperatures.to_option(),
            networks: networks.to_option(),
        }
    }
}

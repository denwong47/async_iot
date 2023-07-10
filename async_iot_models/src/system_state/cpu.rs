use std::{io, thread::sleep, time::Duration};

use serde_json::{self, value::Value};
use sysinfo::{self, CpuExt, SystemExt};
use systemstat::{self, Platform};

use super::SystemState;
use crate::{results, traits::ResultToOption};

/// Convert the CPU details into our proprietary JSON format.
fn cpu_to_json(cpu: &sysinfo::Cpu) -> Value {
    serde_json::json!(
        {
            "name": cpu.name(),
            "frequency": cpu.frequency(),
            "usage": cpu.cpu_usage(),
            "vendorId": cpu.vendor_id(),
            "brand": cpu.brand(),
        }
    )
}

/// Convert the CPU details into a mapping.
fn cpu_to_map(cpu: &sysinfo::Cpu) -> serde_json::Map<String, Value> {
    if let Value::Object(mapping) = cpu_to_json(cpu) {
        mapping
    } else {
        serde_json::Map::new()
    }
}

/// Get the current CPU details.
pub fn cpu(sys: &SystemState) -> results::ExtendedResult<Value, io::Error> {
    let mut warnings = Vec::new();
    let cpu_globals = sys.sysinfo.global_cpu_info();

    let cpu_load = sys.systemstat.cpu_load_aggregate().and_then(|cpu| {
        sleep(Duration::from_micros(50000));
        cpu.done()
    });

    let cpu_cores: Vec<Value> = sys.sysinfo.cpus().iter().map(cpu_to_json).collect();

    let mut json = cpu_to_map(cpu_globals);
    macro_rules! expand_fields {
        (
            $((
                $field: literal,
                $expr: expr
            )),+$(,)?
        ) => {
            $(
                json.insert(
                    $field.to_owned(),
                    serde_json::to_value(
                        $expr
                        .or_else(
                            |err| {
                                warnings.push(
                                    format!(
                                        "Failed to get '{}': {}",
                                        $field,
                                        err
                                    )
                                );
                                Err(err)
                            }
                        )
                        .to_option()
                    )
                    .unwrap_or_else(
                        |err|
                            format!(
                                "Cannot serialize '{}' field: {}",
                                $field,
                                err
                            )
                            .into()
                    )
                );
            )*
        };
    }

    expand_fields!(
        (
            "physicalCoreCounts",
            sys.sysinfo
                .physical_core_count()
                .ok_or("Cannot get CPU physical core count.")
        ),
        ("core", serde_json::to_value(&cpu_cores)),
        ("load", cpu_load),
    );

    results::ExtendedResult::Ok(Value::Object(json)).with_warnings(warnings)
}

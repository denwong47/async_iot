use std::{thread::sleep, time::Duration};

use serde_json::{self, value::Value};
use sysinfo::{self, CpuExt, SystemExt};
use systemstat::{self, Platform};

use super::SystemState;
use crate::{results, traits::FromWithKey};

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

fn cpu_to_result_json(key: &str, cpu: &sysinfo::Cpu) -> results::ResultJsonEntry {
    results::ResultJsonEntry::new_mapping(key.to_owned(), results::ResultState::Ok).with_children(
        vec![
            results::ResultJsonEntry::new_scalar(
                "name".to_owned(),
                results::ResultState::Ok,
                Some(cpu.name()),
            ),
            results::ResultJsonEntry::new_scalar(
                "frequency".to_owned(),
                results::ResultState::Ok,
                Some(cpu.frequency()),
            ),
            results::ResultJsonEntry::new_scalar(
                "usage".to_owned(),
                results::ResultState::Ok,
                Some(cpu.cpu_usage()),
            ),
            results::ResultJsonEntry::new_scalar(
                "vendorId".to_owned(),
                results::ResultState::Ok,
                Some(cpu.vendor_id()),
            ),
            results::ResultJsonEntry::new_scalar(
                "brand".to_owned(),
                results::ResultState::Ok,
                Some(cpu.brand()),
            ),
        ],
    )
}

/// Get the current CPU details.
pub fn cpu(key: &str, sys: &SystemState) -> results::ResultJsonEntry {
    let cpu_globals = sys.sysinfo.global_cpu_info();

    let cpu_load = sys.systemstat.cpu_load_aggregate().and_then(|cpu| {
        sleep(Duration::from_micros(50000));
        cpu.done()
    });

    let cpu_cores: Vec<Value> = sys.sysinfo.cpus().iter().map(cpu_to_json).collect();

    cpu_to_result_json(key, cpu_globals).with_children(vec![
        results::ResultJsonEntry::from_with_key(
            "physicalCoresCount",
            sys.sysinfo
                .physical_core_count()
                .and_then(|int| Some(serde_json::Value::Number(int.into())))
                .ok_or(format!("Unable to get CPU physical core count.")),
        ),
        FromWithKey::<Result<Value, String>>::from_with_key(
            "cores",
            Ok(serde_json::Value::Array(cpu_cores)),
        ),
        results::ResultJsonEntry::from_with_key(
            "load",
            cpu_load
                .map_err(|err| err.to_string())
                .and_then(|load| serde_json::to_value(load).map_err(|err| err.to_string()))
                .map_err(|err| format!("Unable to get CPU load: {err}")),
        ),
    ])
}

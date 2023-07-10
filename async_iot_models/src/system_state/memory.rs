use std::io;

use serde_json::{self, value::Value};
use sysinfo::{self, SystemExt};

use super::SystemState;
use crate::results;

/// Get details about current memory usage.
pub fn memory(sys: &SystemState) -> results::ExtendedResult<Value, io::Error> {
    let physical = serde_json::json!(
        {
            "total": sys.sysinfo.total_memory(),
            "available": sys.sysinfo.available_memory(),
            "free": sys.sysinfo.free_memory(),
            "used": sys.sysinfo.used_memory(),
            "percent_used": sys.sysinfo.used_memory() as f64 / sys.sysinfo.total_memory() as f64,
        }
    );

    let swap = serde_json::json!(
        {
            "total": sys.sysinfo.total_swap(),
            "free": sys.sysinfo.free_swap(),
            "used": sys.sysinfo.used_swap(),
            "percent_used": sys.sysinfo.used_swap() as f64 / sys.sysinfo.total_swap() as f64,
        }
    );

    results::ExtendedResult::Ok(serde_json::json!(
        {
            "physical": physical,
            "swap": swap,
        }
    ))
}

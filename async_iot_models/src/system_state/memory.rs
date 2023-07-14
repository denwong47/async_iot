use serde_json;
use sysinfo::{self, SystemExt};

use super::SystemState;
use crate::{results, traits::FromWithKey};

/// Get details about current memory usage.
pub fn memory(key: &str, sys: &SystemState) -> results::ResultJsonEntry {
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

    results::ResultJsonEntry::new_mapping(key.to_owned(), results::ResultState::Ok).with_children(
        vec![
            results::ResultJsonEntry::from_with_key("physical", physical),
            results::ResultJsonEntry::from_with_key("swap", swap),
        ],
    )
}

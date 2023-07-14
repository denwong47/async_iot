use serde_json;
use sysinfo::{self, SystemExt};

use super::SystemState;
use crate::results;

/// Get the system details.
pub fn system(key: &str, sys: &SystemState) -> results::ResultJsonEntry {
    results::ResultJsonEntry::from_value(
        key,
        serde_json::json!(
            {
                "name": sys.sysinfo.name(),
                "osVersion": sys.sysinfo.os_version(),
                "osKernelVersion": sys.sysinfo.kernel_version(),
                "hostName": sys.sysinfo.host_name(),
            }
        ),
    )
}

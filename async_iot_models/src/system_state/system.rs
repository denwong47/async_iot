use std::io;

use serde_json::{self, value::Value};
use sysinfo::{self, SystemExt};

use super::SystemState;
use crate::results;

/// Get the system details.
pub fn system(sys: &SystemState) -> results::ExtendedResult<Value, io::Error> {
    results::ExtendedResult::Ok(serde_json::json!(
        {
            "name": sys.sysinfo.name(),
            "osVersion": sys.sysinfo.os_version(),
            "osKernelVersion": sys.sysinfo.kernel_version(),
            "hostName": sys.sysinfo.host_name(),
        }
    ))
}

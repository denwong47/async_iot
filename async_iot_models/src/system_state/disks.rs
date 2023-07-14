use serde_json::{self, value::Value};
use sysinfo::{self, DiskExt, SystemExt};

use super::SystemState;
use crate::results;

/// Get details about current disks usage.
pub fn disks(key: &str, sys: &SystemState) -> results::ResultJsonEntry {
    results::ResultJsonEntry::new_scalar(
        key.to_owned(),
        results::ResultState::Ok,
        Some(
            Value::Array(
                sys.sysinfo
                    .disks()
                    .iter()
                    .map(
                        |disk| {
                            let used = disk.total_space() - disk.available_space();

                            serde_json::json!(
                                {
                                    "kind": disk.kind(),
                                    "name": disk.name().to_str().unwrap_or("(unknown)"),
                                    "file_system": std::str::from_utf8(disk.file_system()).unwrap_or("(unknown)"),
                                    "mount_point": disk.mount_point(),
                                    "total": disk.total_space(),
                                    "used": used,
                                    "free": disk.available_space(),
                                    "percent_used": used as f64 / disk.total_space() as f64,
                                    "is_removable": disk.is_removable(),
                                }
                            )
                        }
                    )
                    .collect()
            )
        )
    )
}

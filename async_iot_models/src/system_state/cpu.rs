use std::{io, thread::sleep, time::Duration};

use systemstat::{self, Platform};

use crate::results;

/// Get the current CPU load.
pub fn cpu_load(
    sys: &systemstat::System,
) -> results::ExtendedResult<systemstat::CPULoad, io::Error> {
    sys.cpu_load_aggregate()
        .and_then(|cpu| {
            sleep(Duration::from_micros(10000));
            cpu.done()
        })
        .into()
}

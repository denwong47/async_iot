use std::{collections::HashMap, io};

use systemstat::{self, Platform};

use crate::results;

/// Get temperatures of this machine.
pub fn temperatures(
    sys: &systemstat::System,
) -> results::ExtendedResult<HashMap<&'static str, f32>, io::Error> {
    let mut map = HashMap::new();
    let mut warnings = Vec::new();

    macro_rules! expand_fetchers {
        (
            $((
                $key: ident,
                $func: expr
            )),*
            $(,)?
        ) => {
            $(
            match $func(sys) {
                Ok(value) => {map.insert(stringify!($key), value);},
                Err(msg) => warnings.push(msg.to_string()),
            }
            )*
        };
    }

    expand_fetchers!((cpu, cpu_temp),);

    results::ExtendedResult::Ok(map).with_warnings(warnings)
}

/// Internal function to fetch CPU temperature.
fn cpu_temp(sys: &systemstat::System) -> io::Result<f32> {
    sys.cpu_temp()
}

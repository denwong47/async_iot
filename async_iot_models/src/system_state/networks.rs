use serde::Serialize;
use serde_json::json;
use std::{collections::HashMap, io};
use systemstat::{self, data::IpAddr, NetworkAddrs, NetworkStats, Platform};

use serde_json;

use crate::{exit_codes, results::ExtendedResult};

/// A state for a network interface.
#[derive(Clone, Debug, Serialize)]
pub struct InterfaceState {
    address_v4: Vec<serde_json::Value>,
    address_v6: Vec<serde_json::Value>,
    stats: Option<NetworkStats>,
}

impl InterfaceState {
    /// Create a new instance of [`InterfaceState`] from raw [`systemstat`] components.
    fn try_from_systemstat(
        name: &str,
        addrs: Vec<NetworkAddrs>,
        stats_result: io::Result<NetworkStats>,
    ) -> ExtendedResult<InterfaceState, io::Error> {
        let mut address_v4 = Vec::new();
        let mut address_v6 = Vec::new();

        let mut warnings = Vec::new();

        addrs
            .into_iter()
            .for_each(|addr| match (addr.addr, addr.netmask) {
                (IpAddr::V4(ip), IpAddr::V4(mask)) => address_v4.push(json!({
                    "addr": ip,
                    "mask": mask
                })),
                (IpAddr::V6(ip), IpAddr::V6(mask)) => address_v6.push(json!({
                    "addr": ip,
                    "mask": mask
                })),
                _ => (),
            });

        let stats = match stats_result {
            Ok(stats) => Some(stats),
            Err(err) => {
                warnings.push(format!(
                    "Cannot get a network address for interface `{name}`: {err}",
                ));
                None
            }
        };

        ExtendedResult::Ok(InterfaceState {
            address_v4,
            address_v6,
            stats,
        })
        .with_warnings(warnings)
    }
}

/// Get the current networks using systemstat.
pub fn networks(
    sys: &systemstat::System,
) -> ExtendedResult<HashMap<String, Option<InterfaceState>>, io::Error> {
    let try_networks = sys.networks();

    match try_networks {
        Ok(networks) => {
            let (map, warnings) = networks
                .into_iter()
                .map(|(_, network)| {
                    (
                        network.name.clone(),
                        network.addrs,
                        sys.network_stats(&network.name),
                    )
                })
                .fold(
                    (HashMap::new(), Vec::new()),
                    |(mut map, mut warnings), (name, addrs, stats)| {
                        let result = InterfaceState::try_from_systemstat(&name, addrs, stats);

                        map.insert(
                            name,
                            match result {
                                ExtendedResult::Ok(state) => Some(state),
                                ExtendedResult::WithWarnings(state, mut _warnings) => {
                                    warnings.append(&mut _warnings);
                                    Some(state)
                                }
                                ExtendedResult::Err(_, err) => {
                                    warnings.push(err.to_string());
                                    None
                                }
                            },
                        );

                        (map, warnings)
                    },
                );

            ExtendedResult::Ok(map).with_warnings(warnings)
        }
        Err(err) => ExtendedResult::Err(exit_codes::SYSTEM_READ_FAILURE, err),
    }
}

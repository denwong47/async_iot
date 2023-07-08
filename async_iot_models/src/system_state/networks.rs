use serde::Serialize;
use std::{collections::HashMap, io};
use systemstat::{self, data::IpAddr, NetworkAddrs, NetworkStats, Platform};

use crate::results::ExtendedResult;

/// A state for a network interface.
#[derive(Clone, Debug, Serialize)]
pub struct InterfaceState {
    address_v4: Vec<NetworkAddrs>,
    address_v6: Vec<NetworkAddrs>,
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

        addrs.into_iter().for_each(|addr| match addr.addr {
            IpAddr::V4(_) => address_v4.push(addr),
            IpAddr::V6(_) => address_v6.push(addr),
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
                                ExtendedResult::Err(err) => {
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
        Err(err) => ExtendedResult::Err(err),
    }
}

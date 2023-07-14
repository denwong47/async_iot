use serde::Serialize;
use serde_json::json;
use std::io;
use systemstat::{self, data::IpAddr, NetworkAddrs, NetworkStats, Platform};

use serde_json;

use super::SystemState;
use crate::{
    results::{self, ExtendedResult},
    traits::FromWithKey,
};

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
pub fn networks(key: &str, sys: &SystemState) -> results::ResultJsonEntry {
    let try_networks = sys.systemstat.networks();

    match try_networks {
        Ok(networks) => networks
            .into_iter()
            .map(|(_, network)| {
                (
                    network.name.clone(),
                    network.addrs,
                    sys.systemstat.network_stats(&network.name),
                )
            })
            .fold(
                results::ResultJsonEntry::new_mapping(key.to_owned(), results::ResultState::Ok),
                |entry, (name, addrs, stats)| {
                    let result = InterfaceState::try_from_systemstat(&name, addrs, stats);

                    entry.add_child_entry(results::ResultJsonEntry::from_with_key(&name, result))
                },
            ),
        Err(err) => results::ResultJsonEntry::from_err(key, err),
    }
}

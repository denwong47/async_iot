use async_iot_models::system_state::SystemState;

use serde::{Deserialize, Serialize};

#[cfg(feature = "shellyv1")]
use crate::shellyv1;

pub struct Device {
    kind: String,
    path: String,
    params: DeviceType,
}

pub enum DeviceType {
    SystemState(SystemState),

    #[cfg(feature = "shellyv1")]
    Shelly1(shellyv1::devices::Shelly1<false>),

    #[cfg(feature = "shellyv1")]
    Shelly1PM(shellyv1::devices::Shelly1<true>),

    #[cfg(feature = "shellyv1")]
    Shelly1L(shellyv1::devices::Shelly1L),
}

// impl Deserialize for Device {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//     }
// }

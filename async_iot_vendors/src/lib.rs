pub mod error;

mod device_type;
pub use device_type::*;

#[cfg(feature = "remote_system_state")]
pub mod remote_system_state;

#[cfg(feature = "shellyv1")]
pub mod shellyv1;

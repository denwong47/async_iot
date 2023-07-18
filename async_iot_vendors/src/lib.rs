pub mod error;

mod device_type;
pub use device_type::*;

#[cfg(feature = "shellyv1")]
pub mod shellyv1;

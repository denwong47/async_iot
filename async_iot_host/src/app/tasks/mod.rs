#[macro_use]
pub mod futureless;

mod termination;
pub(crate) use termination::*;

#[cfg(feature = "system_state")]
mod system_state;
#[cfg(feature = "system_state")]
pub(crate) use system_state::*;

#[cfg(feature = "shellyv1")]
mod shellyv1;
#[cfg(feature = "shellyv1")]
pub(crate) use shellyv1::*;

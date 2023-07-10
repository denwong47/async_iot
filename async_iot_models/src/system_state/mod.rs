mod base;
pub use base::*;

pub mod cpu;
pub mod disks;
pub mod memory;
pub mod temperatures;

mod networks;
pub use networks::InterfaceState;

pub mod system;

#[cfg(test)]
mod tests;

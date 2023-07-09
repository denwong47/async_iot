pub use async_iot_models::config::*;

pub const DEFAULT_PORT: u16 = 4088;
pub const SYSTEM_STATE_REFRESH_RATE: u64 = 5;

#[cfg(feature = "detailed_logging")]
pub const LOG_LATEST_VISITS: usize = 10;

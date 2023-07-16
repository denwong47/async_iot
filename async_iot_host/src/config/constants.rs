pub use async_iot_models::config::*;

pub const DEFAULT_CONFIG_PATH: &str = "./host.json";
pub const DEFAULT_ADDR: &str = "0.0.0.0";
pub const DEFAULT_PORT: u16 = 4088;
pub const SYSTEM_STATE_REFRESH_RATE: u64 = 5;

#[cfg(feature = "detailed_logging")]
pub const LOG_LATEST_VISITS: usize = 10;

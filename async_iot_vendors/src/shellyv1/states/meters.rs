use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TimestampSeconds};

use time::OffsetDateTime;

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Meters<const ID: u8> {
    pub power: u16,
    pub is_valid: bool,
    pub overpower: Option<bool>,
    #[serde_as(as = "TimestampSeconds<i64>")]
    pub timestamp: OffsetDateTime,
    pub counters: Vec<u64>,
    pub total: u64,
}

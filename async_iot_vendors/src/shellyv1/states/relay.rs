use ::serde::{Deserialize, Serialize};
use serde_with::{serde_as, TimestampSeconds};
use time::OffsetDateTime;

use super::Turn;
use async_iot_models::traits;

#[allow(dead_code)]
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Relay<const ID: u8> {
    pub ison: bool,
    pub has_time: bool,
    #[serde_as(as = "TimestampSeconds<i64>")]
    pub timer_started: OffsetDateTime,
    pub timer_duration: u64,
    pub timer_remaining: u64,
    pub overpower: Option<bool>,
    pub source: String,
}

impl<const ID: u8> traits::markers::ResponseSchema for Relay<{ ID }> {}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RelayGet<const ID: u8> {
    turn: Option<Turn>,
    timer: Option<u64>,
}

impl<const ID: u8> traits::markers::QuerySchema for RelayGet<{ ID }> {}

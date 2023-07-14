use ::serde::{Deserialize, Serialize};
use enum_index::*;
use serde_with::{serde_as, TimestampSeconds};

use async_iot_models::traits::{self, markers::QuerySchema};

use super::Turn;
use time::OffsetDateTime;

#[derive(Clone, Debug, EnumIndex)]
#[index_type(String)]
pub enum LightMode {
    #[index("colour")]
    Color,

    #[index("white")]
    White,
}

#[derive(Clone, Debug, EnumIndex)]
#[index_type(u8)]
pub enum LightEffect {
    #[index(0)]
    Off,

    #[index(1)]
    MeteorShower,

    #[index(2)]
    GradualChange,

    #[index(3)]
    Flash,

    #[index(4)]
    Breath,

    #[index(5)]
    OnOffGradual,

    #[index(6)]
    RedGreenChange,
}

#[allow(dead_code)]
#[serde_as]
#[derive(Clone, Serialize, Deserialize)]
pub struct Light<const ID: u8> {
    /// Whether the channel is turned ON or OFF
    ison: bool,

    /// Source of the last command
    source: String,

    /// Whether a timer is currently armed for this channel
    has_timer: bool,

    /// Unix timestamp of timer start; 0 if timer inactive or time not synced
    #[serde_as(as = "TimestampSeconds<i64>")]
    timer_started: OffsetDateTime,

    /// Timer duration, s
    timer_duration: u64,

    /// experimental. If there is an active timer, shows seconds until timer elapses; 0 otherwise
    timer_remaining: u64,

    /// Currently configured mode
    mode: LightMode,

    /// Red brightness, 0..255, applies in mode="color"
    red: u8,

    /// Green brightness, 0..255, applies in mode="color"
    green: u8,

    /// Blue brightness, 0..255, applies in mode="color"
    blue: u8,

    /// White brightness, 0..255, applies in mode="color"
    white: u8,

    /// Gain for all channels, 0..100, applies in mode="color"
    gain: u8,

    /// Color temperature in K, 3000..6500, applies in mode="white"
    temp: u16,

    /// Brightness, 0..100, applies in mode="white"
    brightness: u8,

    /// Currently applied effect, description
    effect: LightEffect,
}

impl<const ID: u8> traits::markers::ResponseSchema for Light<{ ID }> {}

#[allow(dead_code)]
#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LightGet<const ID: u8> {
    /// Accepted values are white and color
    mode: Option<LightMode>,

    /// Whether a timer is currently armed for this channel
    timer: Option<u64>,

    /// Command to turn on, off or toggle
    turn: Option<Turn>,

    /// Red brightness, 0..255, applies in mode="color"
    red: Option<u8>,

    /// Green brightness, 0..255, applies in mode="color"
    green: Option<u8>,

    /// Blue brightness, 0..255, applies in mode="color"
    blue: Option<u8>,

    /// White brightness, 0..255, applies in mode="color"
    white: Option<u8>,

    /// Gain for all channels, 0..100, applies in mode="color"
    gain: Option<u8>,

    /// Color temperature in K, 3000..6500, applies in mode="white"
    temp: Option<u16>,

    /// Brightness, 0..100, applies in mode="white"
    brightness: Option<u8>,

    /// Currently applied effect, description
    effect: Option<LightEffect>,
}

impl<const ID: u8> QuerySchema for LightGet<{ ID }> {}

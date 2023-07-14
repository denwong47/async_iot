mod actions;
mod ap;
mod cloud;
mod login;
mod sta;

pub use actions::*;
pub use ap::*;
pub use cloud::*;
pub use login::*;
pub use sta::*;

use async_iot_models::traits;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Clone, Serialize, Deserialize)]
pub struct Settings {
    pub actions: actions::SettingsActions,
    pub ap: ap::SettingsAp,
    pub cloud: cloud::SettingsCloud,
    pub login: login::SettingsLogin,
    pub sta: sta::SettingsSta,
}

impl traits::markers::ResponseSchema for Settings {}

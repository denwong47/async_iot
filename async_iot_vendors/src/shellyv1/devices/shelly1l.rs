use serde::{Deserialize, Serialize};
use url::Url;

use async_iot_models::{
    end_point_type,
    traits::{self, BuildWith},
    LocalError,
};

use super::super::{auth, states};

#[allow(dead_code)]
#[derive(Clone, Serialize, Deserialize)]
pub struct Shelly1L {
    addr: Url,
    auth: Option<auth::BasicAuthentication>,
    shelly: states::shelly::Shelly,
    settings: states::settings::Settings,
    light0: states::light::Light<0>,
}

impl traits::ClientTransformer for Shelly1L {
    fn transform(&self, builder: reqwest::ClientBuilder) -> reqwest::ClientBuilder {
        builder
    }
}

impl traits::RequestTransformer for Shelly1L {
    fn transform(&self, builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        builder.build_with(&self.auth)
    }
}

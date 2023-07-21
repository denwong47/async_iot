use async_trait::async_trait;
use reqwest::Client;
use std::sync::{Arc, RwLock};
use url::Url;

// TODO Remove
#[allow(unused_imports)]
use async_iot_models::{
    auth, end_point_type, results, system_state,
    traits::{self, BuildWith, CanGet, EndPoint, FromWithKey},
    LocalError,
};

#[allow(dead_code)]
pub struct RemoteSystemState {
    _cache: RwLock<Option<results::ResultJson>>,
    auth: Option<auth::BasicAuthentication>,
    client: Arc<Client>,
    addr: Url,
}

impl traits::ClientTransformer for RemoteSystemState {
    fn transform(&self, builder: reqwest::ClientBuilder) -> reqwest::ClientBuilder {
        builder
    }
}

impl traits::RequestTransformer for RemoteSystemState {
    fn transform(&self, builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        builder.build_with(&self.auth)
    }
}

#[async_trait]
#[allow(unused_variables)]
impl traits::HasState for RemoteSystemState {
    async fn try_get(&self, keys: &[&str]) -> Result<results::ResultJson, LocalError> {
        todo!()
    }

    async fn try_all(&self) -> Result<results::ResultJson, LocalError> {
        todo!()
    }

    fn available_keys() -> Vec<&'static str> {
        system_state::SystemState::available_keys()
    }
}

#[async_trait]
impl traits::HasCachedState for RemoteSystemState {
    async fn locked_cache<'a>(
        &'a self,
    ) -> &'a RwLock<Option<async_iot_models::results::ResultJson>> {
        &self._cache
    }
}

impl RemoteSystemState {
    pub fn new(addr: Url, auth: Option<auth::BasicAuthentication>) -> Result<Self, LocalError> {
        Ok(Self {
            _cache: RwLock::new(None),
            auth: auth,
            client: Self::build_client()?,
            addr: addr,
        })
    }

    pub fn build_client() -> Result<Arc<Client>, LocalError> {
        // Store the cookies
        Client::builder()
            .cookie_store(true)
            .build()
            .map(Arc::new)
            .map_err(LocalError::from)
    }
}

use async_trait::async_trait;
use reqwest::Client;
use std::sync::{Arc, RwLock};
use url::Url;

use async_iot_models::{
    end_point_type, results,
    traits::{self, BuildWith, CanGet, EndPoint, FromWithKey},
    LocalError,
};

use super::super::{auth, states};

#[allow(dead_code)]
pub struct Shelly1<const PM: bool> {
    _cache: RwLock<Option<results::ResultJson>>,
    auth: Option<auth::BasicAuthentication>,
    client: Arc<Client>,
    addr: Url,
}

impl<const PM: bool> traits::ClientTransformer for Shelly1<{ PM }> {
    fn transform(&self, builder: reqwest::ClientBuilder) -> reqwest::ClientBuilder {
        builder
    }
}

impl<const PM: bool> traits::RequestTransformer for Shelly1<{ PM }> {
    fn transform(&self, builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        builder.build_with(&self.auth)
    }
}

macro_rules! expand_fields {
    ($(($field:literal, $query:path, $response:path)),*$(,)?) => {
        #[async_trait]
        impl<const PM: bool>
            traits::HasState for Shelly1<{ PM }>
        {
            /// Get a subset of a state matching the supplied keys.
            ///
            /// This method returns a [`Result`], which will be rendered as a [`results::ResultJson`] by [`get()`].
            async fn try_get(&self, keys: &[&str]) -> Result<results::ResultJson, LocalError> {
                let mut entries: Vec<results::ResultJsonEntry> = Vec::new();

                $(
                    if keys.contains(&$field) {
                        entries.push(
                            results::ResultJsonEntry::from_with_key(
                                $field,
                                CanGet::<
                                    $query,
                                    traits::markers::NotSupported,
                                    $response,
                                >::get(
                                    self,
                                    &self.client,
                                    Some(&states::relay::RelayGet::<0>::default())
                                )
                                .await?
                            )
                        );
                    }
                )*

                Ok(results::ResultJson::new().with_children(entries))
            }

            /// Get a full state with all the available keys.
            ///
            /// This method returns a [`Result`], which will be rendered as a [`results::ResultJson`] by [`get()`].
            async fn try_all(&self) -> Result<results::ResultJson, LocalError> {
                self.try_get(&self.available_keys()).await
            }

            /// Get all the available keys for [`get()`] and [`try_get()`].
            fn available_keys(&self) -> Vec<&str> {
                vec![$($field,)*]
            }
        }

    };
}

expand_fields!(
    ("relay0", states::relay::RelayGet<0>, states::relay::Relay<0>),
);

#[async_trait]
impl<const PM: bool> traits::HasCachedState for Shelly1<{ PM }> {
    async fn locked_cache<'a>(
        &'a self,
    ) -> &'a RwLock<Option<async_iot_models::results::ResultJson>> {
        &self._cache
    }
}

impl<const PM: bool> Shelly1<{ PM }> {
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

impl<const PM: bool, const ID: u8>
    EndPoint<
        end_point_type::Get,
        states::relay::RelayGet<ID>,
        traits::markers::NotSupported,
        states::relay::Relay<ID>,
    > for Shelly1<{ PM }>
{
    fn url(&self) -> Result<String, LocalError> {
        self.addr
            .join(&format!("relay/{id}", id = ID))
            .map(|url| url.to_string())
            .map_err(LocalError::from)
    }
}

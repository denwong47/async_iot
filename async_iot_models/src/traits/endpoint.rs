use async_trait::async_trait;
use reqwest::{Client, RequestBuilder};

use super::{markers, ClientTransformer, RequestTransformer};
use crate::error::LocalError;
pub(crate) use crate::traits::markers::EndPointType;

pub mod end_point_type {
    use super::EndPointType;
    use reqwest::{Client, RequestBuilder};

    macro_rules! expand_types {
        ($(($type:ident, $meth:ident)),*$(,)?) => {
            $(
                pub struct $type {}
                impl EndPointType for $type {
                    fn request_builder(client: &Client, url: &str) -> RequestBuilder {
                        client.$meth(url)
                    }
                }
            )*

        };
    }

    expand_types!((Get, get), (Post, post), (Patch, patch), (Delete, delete),);
}

#[async_trait]
pub trait EndPoint<T, Q, D, R>: ClientTransformer + RequestTransformer
where
    T: EndPointType,
    Q: markers::QuerySchema,
    D: markers::PostSchema,
    R: markers::ResponseSchema,
    Self: Sized,
{
    fn url(&self) -> Result<String, LocalError>;

    fn build_request(
        &self,
        client: &Client,
        query: Option<&Q>,
        data: Option<&D>,
    ) -> Result<RequestBuilder, LocalError> {
        let mut builder = T::request_builder(client, &self.url()?);

        builder = RequestTransformer::transform(self, builder);

        // If a query is used, include it.
        if let Some(query_inner) = query {
            builder = builder.query(query_inner);
        }

        // If data is provided, include it.
        if let Some(data_inner) = data {
            builder = builder.json(data_inner);
        }

        Ok(builder)
    }

    async fn send(
        &self,
        client: &Client,
        query: Option<&Q>,
        data: Option<&D>,
    ) -> Result<R, LocalError> {
        let result = self.build_request(client, query, data)?.send().await;

        {
            match result {
                Ok(response) => response.json::<R>().await.map_err(LocalError::from),
                Err(err) => Err(err.into()),
            }
        }
    }
}

use async_trait::async_trait;
use reqwest::Client;

use super::markers;
use super::{end_point_type::Post, EndPoint};

use crate::error::LocalError;

#[async_trait]
pub trait CanPost<Q, D, R>
where
    Q: markers::QuerySchema,
    D: markers::PostSchema,
    R: markers::ResponseSchema,
{
    async fn post(
        &self,
        client: &Client,
        query: Option<&Q>,
        data: Option<&D>,
    ) -> Result<R, LocalError>;
}

#[async_trait]
impl<T, Q, D, R> CanPost<Q, D, R> for T
where
    T: EndPoint<Post, Q, D, R> + Sync,
    Q: markers::QuerySchema,
    D: markers::PostSchema,
    R: markers::ResponseSchema,
{
    async fn post(
        &self,
        client: &Client,
        query: Option<&Q>,
        data: Option<&D>,
    ) -> Result<R, LocalError> {
        self.send(client, query, data).await
    }
}

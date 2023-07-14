use async_trait::async_trait;
use reqwest::Client;

use super::markers;
use super::{end_point_type::Get, EndPoint};

use crate::error::LocalError;

#[async_trait]
pub trait CanGet<Q, D, R>
where
    Q: markers::QuerySchema,
    D: markers::PostSchema,
    R: markers::ResponseSchema,
{
    async fn get(&self, client: &Client, query: Option<&Q>) -> Result<R, LocalError>;
}

#[async_trait]
impl<T, Q, D, R> CanGet<Q, D, R> for T
where
    T: EndPoint<Get, Q, D, R> + Sync,
    Q: markers::QuerySchema,
    D: markers::PostSchema,
    R: markers::ResponseSchema,
{
    async fn get(&self, client: &Client, query: Option<&Q>) -> Result<R, LocalError> {
        self.send(client, query, None).await
    }
}

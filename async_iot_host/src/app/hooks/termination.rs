use std::sync::Arc;

use async_trait::async_trait;
use http_types::mime;
use tide::{prelude::*, Endpoint};
use tokio::sync::Notify;

pub struct TerminationHook {
    termination_flag: Arc<Notify>,
}

impl TerminationHook {
    /// Create a new [`TerminationHook`] from a [`Notify`] behind an [`Arc`]
    /// reference.
    pub fn new(termination_flag: Arc<Notify>) -> Self {
        Self { termination_flag }
    }
}

#[async_trait]
impl<State> Endpoint<State> for TerminationHook
where
    State: Clone + Send + Sync + 'static,
{
    async fn call(&self, _req: tide::Request<State>) -> tide::Result {
        self.termination_flag.notify_waiters();

        let response = tide::Response::builder(200)
            .body(json!({
                "_result": {
                    "host": {
                        "status": "ok",
                    }
                },
                "host": {
                    "termination": true,
                }
            }))
            .content_type(mime::JSON)
            .build();

        tide::Result::Ok(response)
    }
}

use std::sync::Arc;

use async_trait::async_trait;
use http_types::mime;
use tide::{self, Endpoint};

use async_iot_models::results;

use crate::app::AppState;

pub struct InfoHook {
    app_state: Arc<AppState>,
}

impl InfoHook {
    /// Create a new [`InfoHook`], used for printing app information.
    pub fn new(app_state: Arc<AppState>) -> Self {
        Self { app_state }
    }
}

#[async_trait]
impl<State> Endpoint<State> for InfoHook
where
    State: Clone + Send + Sync + 'static,
{
    async fn call(&self, req: tide::Request<State>) -> tide::Result {
        // TODO Refactor this to not hard code the path
        drop(self.app_state.log_visit("/info", req.remote()));

        let body = tide::Body::from_json(&results::ResultJson::with_capacity(1).add_result(
            &"info",
            results::ResultState::Ok,
            &self.app_state,
        ))?;

        Ok(tide::Response::builder(200)
            .body(body)
            .content_type(mime::JSON)
            .build())
    }
}

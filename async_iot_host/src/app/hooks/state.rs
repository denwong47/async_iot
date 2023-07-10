use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use http_types::mime;
use tide::{self, prelude::*, Endpoint};

use async_iot_models::{logger, results, system_state::SystemState, traits::ResultToOption};

use crate::app::AppState;
use crate::error::AppError;

pub struct SystemStateHook {
    app_state: Arc<AppState>,
    lock: &'static RwLock<Option<results::ResultJson>>,
    subset: bool,
}

impl SystemStateHook {
    /// Create a new [`SystemStateHook`] from a `'static` [`SystemState`] situated
    /// behind a [`RwLock`].
    ///
    /// Due to the lifetime, [`lazy_static`] is needed to create the instance.
    pub fn new(
        app_state: Arc<AppState>,
        lock: &'static RwLock<Option<results::ResultJson>>,
        subset: bool,
    ) -> Self {
        Self {
            app_state,
            lock,
            subset,
        }
    }
}

#[async_trait]
impl<State> Endpoint<State> for SystemStateHook
where
    State: Clone + Send + Sync + 'static,
{
    async fn call(&self, req: tide::Request<State>) -> tide::Result {
        // TODO Refactor this to not hard code the path
        drop(self.app_state.log_visit("/state", req.remote()));

        let subset = if self.subset {
            req.param("subset").map(|key| vec![key]).to_option()
        } else {
            None
        };

        self.lock
            .read()
            .map_err(|_| tide::Error::new(500, AppError::LockPoisoned("lock for `SystemState`")))
            .and_then(|state_opt| {
                let body = match (state_opt.as_ref(), subset) {
                    (Some(state), Some(subset)) => tide::Body::from_json(&state.get(&subset)),
                    (Some(state), None) => tide::Body::from_json(&state),
                    (None, Some(subset)) => {
                        logger::debug(
                            "Generating new subsetted `SystemState` as global instance is `None`.",
                        );
                        tide::Body::from_json(&SystemState::new().get(&subset))
                    }
                    (None, None) => {
                        logger::debug("Generating new `SystemState` as global instance is `None`.");
                        tide::Body::from_json(&SystemState::new().all())
                    }
                };

                body.map(|body| {
                    tide::Response::builder(200)
                        .body(body)
                        .content_type(mime::JSON)
                        .build()
                })
            })
            .or_else(|err| {
                tide::Body::from_json(&results::ResultJson::with_capacity(1).add_result(
                    &"host",
                    results::ResultState::Err(err.to_string()),
                    json!({}),
                ))
                .and_then(|body| {
                    Ok(tide::Response::builder(500)
                        .body(body)
                        .content_type(mime::JSON)
                        .build())
                })
            })
    }
}

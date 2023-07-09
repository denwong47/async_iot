use std::sync::RwLock;

use async_trait::async_trait;
use http_types::mime;
use tide::{self, prelude::*, Endpoint};

use async_iot_models::{logger, results, system_state::SystemState};

use crate::error::AppError;

pub struct SystemStateHook {
    lock: &'static RwLock<Option<results::ResultJson>>,
}

impl SystemStateHook {
    /// Create a new [`SystemStateHook`] from a `'static` [`SystemState`] situated
    /// behind a [`RwLock`].
    ///
    /// Due to the lifetime, [`lazy_static`] is needed to create the instance.
    pub fn new(lock: &'static RwLock<Option<results::ResultJson>>) -> Self {
        Self { lock }
    }
}

#[async_trait]
impl<State> Endpoint<State> for SystemStateHook
where
    State: Clone + Send + Sync + 'static,
{
    async fn call(&self, req: tide::Request<State>) -> tide::Result {
        match req.remote() {
            Some(remote) => logger::info(&format!("Rendering `SystemState` for '{remote}'.")),
            None => logger::info("Rendering `SystemState` for unknown remote."),
        }

        self.lock
            .read()
            .map_err(|_| tide::Error::new(500, AppError::LockPoisoned("lock for `SystemState`")))
            .and_then(|state_opt| {
                let body = match state_opt.as_ref() {
                    Some(state) => tide::Body::from_json(&state),
                    None => {
                        logger::debug("Generating new `SystemState` as global instance is `None`.");
                        tide::Body::from_json(&SystemState::all())
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

use std::sync::RwLock;

use async_trait::async_trait;
use http_types::mime;
use tide::{prelude::*, Endpoint};

use async_iot_models::{logger, system_state::SystemState};

use crate::error::AppError;

pub struct SystemStateHook {
    lock: &'static RwLock<Option<SystemState>>,
}

impl SystemStateHook {
    /// Create a new [`SystemStateHook`] from a `'static` [`SystemState`] situated
    /// behind a [`RwLock`].
    ///
    /// Due to the lifetime, [`lazy_static`] is needed to create the instance.
    pub fn new(lock: &'static RwLock<Option<SystemState>>) -> Self {
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
            None => logger::info("Rendering `SystemState` for unknown remote.")
        }

        let response = self
            .lock
            .read()
            .map_err(|_| tide::Error::new(500, AppError::LockPoisoned("lock for `SystemState`")))
            .and_then(|state_opt| {
                let body = match state_opt.as_ref() {
                    Some(state) => tide::Body::from_json(&state),
                    None => {
                        logger::debug("Generating new `SystemState` as global instance is `None`.");
                        tide::Body::from_json(&SystemState::default())
                    }
                };

                body.map(|body| {
                    tide::Response::builder(200)
                        .body(body)
                        .content_type(mime::JSON)
                        .build()
                })
            })
            .unwrap_or_else(|err| {
                tide::Response::builder(500)
                    .body(json!({
                        "_result": {
                            "host": {
                                "status": "error",
                                "message": err.to_string()
                            }
                        }
                    }))
                    .content_type(mime::JSON)
                    .build()
            });

        tide::Result::Ok(response)
    }
}

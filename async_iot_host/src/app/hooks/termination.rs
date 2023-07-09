use std::sync::Arc;

use async_trait::async_trait;
use http_types::mime;
use serde::Deserialize;
use tide::{prelude::*, Endpoint};

use async_iot_models::{exit_codes, logger, results};

use super::super::termination::TerminationToken;

use crate::error::AppError;

pub struct TerminationHook {
    token: Arc<TerminationToken>,
}

impl TerminationHook {
    /// Create a new [`TerminationHook`] from a [`TerminationToken`] behind an [`Arc`]
    /// reference.
    pub fn new(token: Arc<TerminationToken>) -> Self {
        Self { token }
    }
}

/// Possible queries for the [`TerminationHook`] [`Endpoint`].
#[derive(Clone, Debug, Deserialize)]
pub struct TerminationQuery {
    pub error: Option<String>,
}

#[async_trait]
impl<State> Endpoint<State> for TerminationHook
where
    State: Clone + Send + Sync + 'static,
{
    async fn call(&self, req: tide::Request<State>) -> tide::Result {
        // Try unpacking the query:
        let response = match req.query::<TerminationQuery>() {
            Ok(query) => {
                let response = tide::Response::builder(200)
                    .body(tide::Body::from_json(
                        &results::ResultJson::with_capacity(1).add_result(
                            &"host",
                            results::ResultState::Ok,
                            json!({"termination": true}
                            ),
                        ),
                    )?)
                    .content_type(mime::JSON)
                    .build();

                if let Some(error) = query.error {
                    // Failure termination.
                    // The remote host had requested this server to terminate. Typically
                    // these are used when a firmware update is rolled out etc.
                    match req.remote() {
                        Some(remote) => {
                            let message =
                                format!("Termination request from '{remote}', app error: {error}.");
                            logger::error(&message);
                        }
                        None => {
                            let message = format!(
                                "Termination request from unknown remote, app error: {error}."
                            );
                            logger::error(&message);
                        }
                    }

                    self.token
                        .notify_failure(
                            exit_codes::REQUESTED_TERMINATION,
                            AppError::RemoteRequestedTermination { message: error },
                        )
                        .await;
                } else {
                    // Successful termination.
                    self.token
                        .notify_with_warnings([match req.remote() {
                            Some(remote) => {
                                let message =
                                    format!("Termination request from '{remote}', app completed.");
                                logger::info(&message);
                                message
                            }
                            None => {
                                let message = String::from(
                                    "Termination request from unknown remote, app completed.",
                                );
                                logger::info(&message);
                                message
                            }
                        }])
                        .await;
                }

                response
            }
            Err(err) => tide::Response::builder(400)
                .body(tide::Body::from_json(
                    &results::ResultJson::with_capacity(1).add_result(
                        &"host",
                        results::ResultState::Err(err.to_string()),
                        json!({"termination": false}),
                    ),
                )?)
                .content_type(mime::JSON)
                .build(),
        };

        tide::Result::Ok(response)
    }
}

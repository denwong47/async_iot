use lazy_static::lazy_static;
use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

#[allow(unused_imports)]
use tide::{self, prelude::*};

use async_iot_models::{
    exit_codes, logger,
    results::{self, ExtendedResult},
};

use super::hooks;
use super::{system_state_task, update_system_state, AppState, TerminationToken};

use crate::{config, error::AppError};

lazy_static! {
    /// This is an example for using doc comment attributes
    static ref SYSTEM_STATE: RwLock<Option<results::ResultJson>> = RwLock::new(None);
}

/// Runs the host app.
pub async fn runs_app(addr: &str, port: Option<u16>) -> results::ExtendedResult<(), AppError> {
    let port = port.unwrap_or(config::DEFAULT_PORT);
    let listen_target = format!("{addr}:{port}");

    logger::info(&format!("Starting app on {listen_target}."));

    // Initialize state.
    logger::debug("Initialising `SystemState`...");
    if let Err(err) = update_system_state(&SYSTEM_STATE) {
        return ExtendedResult::Err(exit_codes::SYSTEM_READ_FAILURE, err);
    }
    logger::debug("Initialising `TerminationToken`...");
    let termination_token = Arc::new(TerminationToken::new());

    // Setting up the App details.
    logger::debug("Initialising `tide::Server`...");
    let mut app = tide::new();

    // TODO Refactor this to deduplicate string literals?
    let app_state = AppState::new(&["/info", "/state", "/terminate"]);
    macro_rules! expand_paths {
        ($((
            $path:literal,
            $end_point:expr
        )),*$(,)?) => {
            $(
                app.at($path).get($end_point);
            )*
        };
    }
    expand_paths!(
        ("/info", hooks::InfoHook::new(Arc::clone(&app_state))),
        (
            "/state",
            hooks::SystemStateHook::new(Arc::clone(&app_state), &SYSTEM_STATE)
        ),
        (
            "/terminate",
            hooks::TerminationHook::new(Arc::clone(&app_state), Arc::clone(&termination_token))
        ),
    );

    // Now we switch between the eternal coroutines:
    // - The HTTP host,
    // - The background loop to update the `SystemState`, and
    // - The task listening to termination events.
    logger::debug("Starting Tokio Select...");
    tokio::select! {
        _ = app.listen(&listen_target) => {
            unreachable!()
        },
        result = system_state_task(
            &SYSTEM_STATE,
            Duration::from_secs(config::SYSTEM_STATE_REFRESH_RATE),
        ) => {
            return result
        },
        result = termination_token.task() => {
            return result
        }
    }
}

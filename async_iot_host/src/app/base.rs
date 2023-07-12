#[allow(unused_imports)]
use std::{sync::Arc, time::Duration};

#[allow(unused_imports)]
use tide::{self, prelude::*};

use async_iot_models::{logger, results, system_state::SystemState, traits::HasCachedState};

use super::{hooks, AppState, TerminationToken};
use crate::{config, error::AppError};

#[allow(unused_imports)]
use crate::feature_gated;

#[cfg(feature = "system_state")]
use super::system_state_task;

#[cfg(feature = "system_state")]
use async_iot_models::exit_codes;

/// Runs the host app.
pub async fn runs_app(addr: &str, port: Option<u16>) -> results::ExtendedResult<(), AppError> {
    let port = port.unwrap_or(config::DEFAULT_PORT);
    let listen_target = format!("{addr}:{port}");

    logger::info(&format!("Starting app on {listen_target}."));

    // Initialize state.
    #[cfg(feature = "system_state")]
    let system_state = {
        logger::debug("Initialising `SystemState`...");
        let system_state = Arc::new(SystemState::new());
        if let Err(err) = system_state.update().await {
            return results::ExtendedResult::Err(exit_codes::SYSTEM_READ_FAILURE, err.into());
        }

        system_state
    };

    logger::debug("Initialising `TerminationToken`...");
    let termination_token = Arc::new(TerminationToken::new());

    // Setting up the App details.
    logger::debug("Initialising `tide::Server`...");
    let mut app = tide::new();

    // TODO Refactor this to deduplicate string literals?
    let app_state = AppState::new();
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
            "/terminate",
            hooks::TerminationHook::new(Arc::clone(&app_state), Arc::clone(&termination_token))
        ),
    );

    #[cfg(feature = "system_state")]
    expand_paths!(
        (
            "/state",
            hooks::SystemStateHook::new(Arc::clone(&app_state), Arc::clone(&system_state), false)
        ),
        (
            "/state/",
            hooks::SystemStateHook::new(Arc::clone(&app_state), Arc::clone(&system_state), false)
        ),
        (
            "/state/:subset",
            hooks::SystemStateHook::new(Arc::clone(&app_state), Arc::clone(&system_state), true)
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
        result = feature_gated!(
            "system_state" => system_state_task(
                Arc::clone(&system_state),
                Duration::from_secs(config::SYSTEM_STATE_REFRESH_RATE),
            )
        ) => {
            return result
        },
        result = termination_token.task() => {
            return result
        }
    }
}

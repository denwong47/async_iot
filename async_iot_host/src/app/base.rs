#[allow(unused_imports)]
use std::{sync::Arc, time::Duration};

#[allow(unused_imports)]
use tide::{self, prelude::*};

#[allow(unused_imports)]
use url::Url;

use async_iot_models::{logger, results};

use super::{hooks, AppState, TerminationToken};
use crate::{config, error::AppError};

#[allow(unused_imports)]
use crate::feature_gated;

#[cfg(feature = "system_state")]
use super::system_state_task;

#[cfg(feature = "system_state")]
use async_iot_models::{exit_codes, system_state::SystemState, traits::HasCachedState};

#[cfg(feature = "shellyv1")]
use async_iot_vendors::shellyv1;

#[cfg(feature = "shellyv1")]
use async_iot_models::auth::BasicAuthentication;

#[cfg(feature = "shellyv1")]
use super::shellyv1_task;

/// Runs the host app.
pub async fn runs_app(config: config::HostConfiguration) -> results::ExtendedResult<(), AppError> {
    let addr = config.addr.unwrap_or(config::DEFAULT_ADDR.to_owned());
    let port = config.port.unwrap_or(config::DEFAULT_PORT);
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

    // Refactor this to use config
    #[cfg(feature = "system_state")]
    expand_paths!(
        (
            "/system",
            hooks::SystemStateHook::new(Arc::clone(&app_state), Arc::clone(&system_state), false)
        ),
        (
            "/system/",
            hooks::SystemStateHook::new(Arc::clone(&app_state), Arc::clone(&system_state), false)
        ),
        (
            "/system/:subset",
            hooks::SystemStateHook::new(Arc::clone(&app_state), Arc::clone(&system_state), true)
        ),
    );

    #[cfg(feature = "shellyv1")]
    let device = Arc::new(
        shellyv1::devices::Shelly1::<false>::new(
            Url::parse("http://yoururl").unwrap(),
            Some(BasicAuthentication::new("username", Some("password"))),
        )
        .unwrap(),
    );

    #[cfg(feature = "shellyv1")]
    expand_paths!((
        // TESTING ONLY
        "/testlight",
        hooks::ShellyV1Hook::new(Arc::clone(&app_state), Arc::clone(&device), false,)
    ));

    // Now we switch between the eternal coroutines:
    // - The HTTP host,
    // - The background loop to update the `SystemState`, and
    // - The task listening to termination events.
    logger::debug("Starting Tokio Select...");
    tokio::select! {
        result = app.listen(&listen_target) => {
            result.map_err(|err| AppError::ServerStartUpError { addr: listen_target, err: err }).into()
        },
        result = feature_gated!(
            "system_state" => system_state_task(
                Arc::clone(&system_state),
                Duration::from_secs(config::SYSTEM_STATE_REFRESH_RATE),
            )
        ) => {
            return result
        },
        result = feature_gated!(
            "shellyv1" => shellyv1_task(
                Arc::clone(&device),
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

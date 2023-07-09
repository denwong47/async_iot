use lazy_static::lazy_static;
use std::{sync::{Arc, RwLock}, time::Duration};

#[allow(unused_imports)]
use tide::{self, prelude::*};

use tokio::sync::Notify;
use async_iot_models::{results, system_state};

use super::hooks;
use super::{system_state_task, update_system_state, termination_task};

use crate::{config, error::AppError};

lazy_static! {
    /// This is an example for using doc comment attributes
    static ref SYSTEM_STATE: RwLock<Option<system_state::SystemState>> = RwLock::new(None);
}

/// Runs the host app.
pub async fn runs_app(addr: &str, port: Option<u16>) -> results::ExtendedResult<(), AppError> {
    let port = port.unwrap_or(config::DEFAULT_PORT);
    let listen_target = format!("{addr}:{port}");

    // Initialize state.
    update_system_state(&SYSTEM_STATE);
    let termination_flag = Arc::new(Notify::new());

    let mut app = tide::new();
    app.at("/info").get(hooks::info);
    app.at("/state")
        .get(hooks::SystemStateHook::new(&SYSTEM_STATE));
    app.at("/terminate").get(hooks::TerminationHook::new(Arc::clone(&termination_flag)));

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
        result = termination_task(Arc::clone(&termination_flag)) => {
            return result
        }
    }
}

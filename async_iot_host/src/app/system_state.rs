use std::{sync::RwLock, time::Duration};

use async_iot_models::{results::ExtendedResult, system_state::SystemState};
use tokio;

use crate::error::AppError;

pub(crate) fn update_system_state(
    container: &RwLock<Option<SystemState>>,
) -> ExtendedResult<(), AppError> {
    let new_state = SystemState::default();

    container
        .write()
        .map(|mut state| {
            *state = Some(new_state);
        })
        .map_err(|_| AppError::LockPoisoned("lock for `SystemState`"))
        .into()
}

/// Refresh a new instance of [`SystemState`] at a configurable frequency.
pub(crate) async fn system_state_task(
    container: &RwLock<Option<SystemState>>,
    interval: Duration,
) -> ExtendedResult<(), AppError> {
    loop {
        tokio::time::sleep(interval).await;

        let result = update_system_state(container);
        match result {
            ExtendedResult::Err(_) => return result,
            _ => println!("Updated `SystemState`."),
        }
    }
}

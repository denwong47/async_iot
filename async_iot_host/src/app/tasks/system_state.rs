use std::{sync::RwLock, time::Duration};

use async_iot_models::{exit_codes, logger, results, system_state::SystemState, traits::HasState};
use tokio;

use crate::error::AppError;

pub(crate) async fn update_system_state(
    container: &RwLock<Option<results::ResultJson>>,
) -> Result<(), AppError> {
    let new_state = SystemState::new().all().await;

    container
        .write()
        .map(|mut state| {
            *state = Some(new_state);
        })
        .map_err(|_| AppError::LockPoisoned("lock for `SystemState`"))
}

/// Refresh a new instance of [`SystemState`] at a configurable frequency.
pub(crate) async fn system_state_task(
    container: &RwLock<Option<results::ResultJson>>,
    interval: Duration,
) -> results::ExtendedResult<(), AppError> {
    loop {
        tokio::time::sleep(interval).await;

        let result = update_system_state(container).await;
        match result {
            Err(err) => return results::ExtendedResult::Err(exit_codes::SYSTEM_READ_FAILURE, err),
            _ => logger::trace("Updated `SystemState`."),
        }
    }
}

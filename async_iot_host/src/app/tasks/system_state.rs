use std::{sync::Arc, time::Duration};

use async_iot_models::{
    exit_codes, logger, results, system_state::SystemState, traits::HasCachedState,
};
use tokio;

use crate::error::AppError;
/// Refresh a new instance of [`SystemState`] at a configurable frequency.
pub(crate) async fn system_state_task(
    state: Arc<SystemState>,
    interval: Duration,
) -> results::ExtendedResult<(), AppError> {
    loop {
        tokio::time::sleep(interval).await;

        let result = state.update().await;
        match result {
            Err(err) => {
                return results::ExtendedResult::Err(exit_codes::SYSTEM_READ_FAILURE, err.into())
            }
            _ => logger::trace("Updated `SystemState`."),
        }
    }
}

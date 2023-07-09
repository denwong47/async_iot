use std::sync::Arc;
use tokio::sync::Notify;

use async_iot_models::results::ExtendedResult;
use crate::error::AppError;

/// Task for handling a graceful termination to the server.
pub async fn termination_task(
    notify: Arc<Notify>,
) -> ExtendedResult<(), AppError> {
    notify.notified().await;
    
    ExtendedResult::Ok(())
}
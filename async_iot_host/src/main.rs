use tokio;

use async_iot_host::{app, error::AppError};

use async_iot_models::{logger, results};

#[tokio::main]
async fn main() -> results::ExtendedResult<(), AppError> {
    logger::info("Starting `async_iot_host`.");

    let result = app::runs_app("0.0.0.0", Some(8080)).await.into();

    logger::info("Terminating `async_iot_host` gracefully.");

    result
}

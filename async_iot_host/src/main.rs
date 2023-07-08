use std::io;

use tokio;

use async_iot_host::{app, error::AppError};

use async_iot_models::{results, system_state::SystemState};

#[tokio::main]
async fn main() -> results::ExtendedResult<(), AppError> {
    println!("Starting `async_iot_host`.");

    app::runs_app("0.0.0.0", Some(8080)).await.into()
}

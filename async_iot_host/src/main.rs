use clap::Parser;
use tokio;

use async_iot_host::{app, args, config, error::AppError};

use async_iot_models::{exit_codes, logger, results};

#[tokio::main]
async fn main() -> results::ExtendedResult<(), AppError> {
    // Resolve configurations
    let cli_args = args::CommandLineParameters::parse();
    let try_config = config::HostConfiguration::try_from_args(&cli_args);

    if let Err(err) = try_config {
        return results::ExtendedResult::Err(exit_codes::CONFIG_INVALID, err);
    }

    let config = try_config.unwrap();

    logger::info("Starting `async_iot_host`.");

    let result = app::runs_app(config).await.into();

    logger::info("Terminating `async_iot_host` gracefully.");

    result
}

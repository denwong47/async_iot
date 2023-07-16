use clap::Parser;
use std::path::PathBuf;

use crate::config;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(version)]
#[command(about)]
#[command(long_about = None)]
pub struct CommandLineParameters {
    /// Binds the host to a custom hosting address.
    #[arg(short, long, value_name = "HOST")]
    pub addr: Option<String>,

    /// Binds the host to this custom port.
    #[arg(short, long, value_name = "PORT")]
    pub port: Option<u16>,

    /// Sets a custom JSON config file. Defaults to `./host.json`.
    #[arg(short, long, value_name = "FILE", default_value = config::DEFAULT_CONFIG_PATH)]
    pub config: PathBuf,
}

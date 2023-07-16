use std::{fs::File, io::BufReader, path::Path};

use serde::{Deserialize, Serialize};
use serde_json::{self, Value as JsonValue};

use crate::{args::CommandLineParameters, error::AppError};

use super::constants::{DEFAULT_ADDR, DEFAULT_PORT};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PathConfiguration {
    device: String,
    path: String,
    values: JsonValue,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HostConfiguration {
    pub addr: Option<String>,
    pub port: Option<u16>,
    pub paths: Vec<PathConfiguration>,
}

impl HostConfiguration {
    pub fn try_from_file<P>(path: P) -> Result<Self, AppError>
    where
        P: AsRef<Path>,
    {
        let file = File::open(&path).map_err(|err| AppError::ConfigNotReadable {
            path: AsRef::<Path>::as_ref(&path).display().to_string(),
            err,
        })?;
        let reader = BufReader::new(file);

        serde_json::from_reader(reader).map_err(|err| AppError::ConfigNotParsable {
            path: AsRef::<Path>::as_ref(&path).display().to_string(),
            err: err,
        })
    }

    pub fn resolve_with_args(mut self, args: &CommandLineParameters) -> Self {
        self.addr = args
            .addr
            .as_ref()
            .or(self.addr.as_ref())
            .map(|s| s.to_owned())
            .or(Some(DEFAULT_ADDR.to_owned()));

        self.port = args.port.or(self.port).or(Some(DEFAULT_PORT));

        self
    }

    pub fn try_from_args(value: &CommandLineParameters) -> Result<Self, AppError> {
        Self::try_from_file(&value.config).map(|config| config.resolve_with_args(value))
    }
}

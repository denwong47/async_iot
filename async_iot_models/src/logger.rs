#[cfg(feature = "jsonl_logging")]
use serde_json;

use time;

use crate::config;

/// An enum for different log levels that appears differently.
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

impl LogLevel {
    /// Return the ANSI 8-bit colour code for this level.
    #[cfg(not(feature = "jsonl_logging"))]
    pub fn ansi_code(&self) -> u8 {
        match self {
            Self::Trace => 8,
            Self::Debug => 27,
            Self::Info => 7,
            Self::Warning => 11,
            Self::Error => 9,
            Self::Critical => 1,
        }
    }

    /// Return the name for this level.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Trace => "trace",
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Critical => "critical",
        }
    }

    /// Return the suffix for this level.
    pub fn suffix(&self) -> &'static str {
        match self {
            Self::Trace => "\u{1b}[1A",
            _ => "",
        }
    }

    /// Wraps the string in the corresponding colour.
    #[cfg(not(feature = "jsonl_logging"))]
    pub fn wraps(&self, bold: bool, text: &str) -> String {
        let ansi_code = self.ansi_code();

        let (bold_prefix, bold_suffix) = if bold {
            ("\u{1b}[1m", "\u{1b}[22m")
        } else {
            ("", "")
        };

        format!("\u{1b}[38:5:{ansi_code}m{bold_prefix}{text}{bold_suffix}\u{1b}[39m")
    }

    /// Format a string for the requested level.
    pub fn format(&self, message: &str) -> String {
        let now = time::OffsetDateTime::now_utc()
            .format(&config::DATETIME_FORMAT)
            .unwrap_or("(Time unavailable)        ".to_owned());

        #[cfg(feature = "jsonl_logging")]
        {
            serde_json::to_string(&serde_json::json!(
                {
                    "timestamp": now,
                    "level": self.name(),
                    "message": message,
                }
            ))
            .unwrap_or("Invalid JSON".to_owned())
        }

        #[cfg(not(feature = "jsonl_logging"))]
        {
            let level = self.wraps(true, &self.name().to_uppercase());
            let message = self.wraps(false, message);
            let suffix = self.suffix();

            let level_len = 9 + 22 + self.ansi_code().to_string().len();

            format!("{now} \u{2502} {level:<level_len$} \u{2502} {message}{suffix}")
        }
    }

    /// Log a string to `stderr` for the requested level.
    pub fn log(&self, message: &str) {
        eprintln!("{}", self.format(message))
    }
}

macro_rules! expand_variants {
    ($((
        $level:ident,
        $func:ident
    )),+$(,)?) => {
        $(
            #[doc = "Write a log line to `stderr` at the "]
            #[doc = stringify!($level)]
            #[doc = " level."]
            pub fn $func(message: &str) {
                LogLevel::$level.log(message)
            }
        )*
    };
}

expand_variants!(
    (Trace, trace),
    (Debug, debug),
    (Info, info),
    (Warning, warning),
    (Error, error),
    (Critical, critical),
);

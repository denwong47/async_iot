use time::{self, format_description::FormatItem};

/// The default datetime format for use in this app.
///
/// This shortened format is compatible with Python's `datetime.fromisoformat`.
pub const DATETIME_FORMAT: &[FormatItem<'_>] = time::macros::format_description!(
    "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:6]"
);

// This creates a `mod _datetime_serde_iso8601`.
time::serde::format_description!(_datetime_serde_iso8601, OffsetDateTime, DATETIME_FORMAT);

/// Re-export the private attributes
pub mod datetime_serde_iso8601 {
    pub use super::_datetime_serde_iso8601::*;
}

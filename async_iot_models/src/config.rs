use lazy_static::lazy_static;
use time;

lazy_static!(
    /// The default datetime format for use in this app.
    ///
    /// This shortened format is compatible with Python's `datetime.fromisoformat`.
    pub static ref DATETIME_FORMAT: &'static [time::format_description::FormatItem<'static>] =
        time::macros::format_description!(
            "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:6]"
        )
    ;
);

use std::{error::Error, process};

use crate::{traits::ResultToOption, exit_codes};

/// An expanded [`std::result::Result`] with an added variant for attaching
/// warnings.
#[derive(Clone, Debug)]
pub enum ExtendedResult<T, E>
where
    E: Error,
{
    Ok(T),
    WithWarnings(T, Vec<String>),
    Err(u8, E),
}

impl<T, E> ExtendedResult<T, E>
where
    E: Error,
{
    /// Attach warnings to this [`SystemStateResult`].
    ///
    /// [`SystemStateResult`]
    pub fn with_warnings(self, mut warnings: Vec<String>) -> Self {
        match self {
            Self::Ok(_) if warnings.is_empty() => self,
            Self::Ok(value) => Self::WithWarnings(value, warnings),
            Self::WithWarnings(value, mut existing) => Self::WithWarnings(value, {
                existing.append(&mut warnings);
                existing
            }),
            Self::Err(_, _) => self,
        }
    }
}

impl<T, E> Default for ExtendedResult<T, E>
where
    T: Default,
    E: Error,
{
    /// For any `T` that supports default, an [`ExtendedResult::Ok(T::default())`]
    /// will be returned.
    fn default() -> Self {
        Self::Ok(T::default())
    }
}

impl<T, E> process::Termination for ExtendedResult<T, E>
where
    E: Error,
{
    /// Allow [`ExtendedResult`] to terminate a `main()`.
    fn report(self) -> process::ExitCode {
        match self {
            Self::Ok(_) => process::ExitCode::SUCCESS,
            Self::WithWarnings(_, warnings) => {
                println!(
                    "\u{1b}[33mThe following warnings are generated upon exit:\n\n{}\u{1b}[39m",
                    warnings
                        .into_iter()
                        .map(|warning| { String::from("- ") + &warning })
                        .fold(String::new(), |lhs, rhs| lhs + &rhs)
                );

                process::ExitCode::SUCCESS
            }
            Self::Err(status, err) => {
                println!("\u{1b}[31mError:\n\n{}\u{1b}[39m", err);

                process::ExitCode::from(status)
            }
        }
    }
}

impl<T, E> From<std::result::Result<T, E>> for ExtendedResult<T, E>
where
    E: Error,
{
    fn from(value: std::result::Result<T, E>) -> Self {
        match value {
            Ok(value) => Self::Ok(value),
            Err(err) => Self::Err(exit_codes::UNKNOWN_FAILURE, err),
        }
    }
}

impl<T, E> ResultToOption<T> for ExtendedResult<T, E>
where
    E: Error,
{
    fn to_option(self) -> Option<T> {
        match self {
            Self::Ok(value) => Some(value),
            Self::WithWarnings(value, _) => Some(value),
            _ => None,
        }
    }
}

/// A generic trait for converting a [`Result`] to [`Option`].
///
/// Useful to let some fields remain upon serialization, even if it is
/// an [`Result::Err`].
pub trait ResultToOption<T> {
    /// Convert an instance to [`Option`].
    fn to_option(self) -> Option<T>;
}

impl<T, E> ResultToOption<T> for Result<T, E> {
    fn to_option(self) -> Option<T> {
        match self {
            Ok(ob) => Some(ob),
            Err(_) => None,
        }
    }
}

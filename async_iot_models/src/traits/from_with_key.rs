#[allow(unused_imports)]
use crate::results::ResultJsonEntry;

/// A trait for [`ResultJsonEntry`] to be created from various types, with a key already
/// embedded.
pub trait FromWithKey<T> {
    fn from_with_key(key: &str, value: T) -> Self;
}

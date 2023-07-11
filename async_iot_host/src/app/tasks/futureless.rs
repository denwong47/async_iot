//! A [`Future`] that never finishes polling.
//!
//! This acts as a placeholder, substituting useful futures with one that does
//! nothing when certain feature flags are not enabled. By never ending to poll,
//! [`tokio::select`] will never be completed any of these [`Futureless`].
//!
//! # Examples
//!
//! ```ignore
//! use async_iot_host::feature_gated;
//!
//! async fn async_func1() -> u8 { 1 }
//! async fn async_func2() -> u8 { 2 }
//!
//! #[tokio::main]
//! async fn main() {
//!     tokio::select!{
//!         _ = feature_gated!("my_feature1" => async_func1()) => { () },
//!         _ = feature_gated!("my_feature2" => async_func2()) => { () },
//!     }
//! }
//! ```

use std::{future::Future, marker::PhantomData};

#[allow(unused_imports)]
use tokio;

/// A [`Future`] that never finishes polling.
pub struct Futureless<T> {
    _phantom: PhantomData<T>,
}

#[allow(dead_code)]
impl<T> Futureless<T> {
    /// Instantiate a new [`Futureless`] instance.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }

    /// An async function that never stops polling.
    pub fn noop() -> impl Future<Output = T> {
        Self::new()
    }
}

impl<T> Future for Futureless<T> {
    type Output = T;

    /// Never stop polling.
    #[allow(unused_variables)]
    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        std::task::Poll::Pending
    }
}

/// A macro to gate certain [`tokio::select`] targets behind feature flags.
#[macro_export]
macro_rules! feature_gated {
    ($feature:literal => $future:expr) => {
        (|| {
            #[cfg(feature = $feature)]
            return $future;

            #[cfg(not(feature = $feature))]
            return crate::app::tasks::futureless::Futureless::noop();
        })()
    };
}

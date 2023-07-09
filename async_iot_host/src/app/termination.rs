use std::{sync::{Arc, RwLock}, mem, ops::DerefMut};
use tokio::sync::Notify;

use async_ctrlc;

use async_iot_models::{exit_codes, results::ExtendedResult};
use crate::error::AppError;

pub struct TerminationToken {
    notify: Arc<Notify>,
    result: RwLock<Option<ExtendedResult<(), AppError>>>,
}

impl TerminationToken {
    /// Create a new [`TerminationToken`] for termination signaling among threads.
    pub fn new() -> Self {
        Self {
            notify: Arc::new(Notify::new()),
            result: RwLock::new(None),
        }
    }

    /// The coroutine to listen for any termination notifications.
    /// 
    /// Typically used as one of the [`tokio::select`] members.
    pub async fn task(self: Arc<Self>) -> ExtendedResult<(), AppError> {
        match async_ctrlc::CtrlC::new() {
            Ok(ctrlc) => {
                tokio::select!{
                    _ = ctrlc => {
                        ExtendedResult::Ok(()).with_warnings(
                            vec!["Termination from command line on host.".to_owned()]
                        )
                    },
                    _ = self.notify.notified() => {
                        self.submit_result(ExtendedResult::default())
                            .await
                            .unwrap_or_default()
                    },
                }
            },
            Err(err) => {
                ExtendedResult::Err(
                    exit_codes::SYSTEM_SETUP_FAILURE,
                    AppError::CtrlCError { message: err.to_string() }
                )
            }
        }

    }

    /// Replace the current result embedded in this [`TerminationToken`] with another
    /// instance. The existing one is returned.
    pub async fn submit_result(
        &self,
        result: ExtendedResult<(), AppError>
    ) -> Option<ExtendedResult<(), AppError>> {
        let mut some_result = Some(result);

        match self.result.write() {
            Ok(mut result_ref) => {
                mem::swap(&mut some_result, result_ref.deref_mut());

                some_result
            },
            Err(_) => {
                panic!(
                    "`TerminationToken` has a poisoned `RwLock` for its `result`. Intended result: {:?}",
                    some_result.unwrap()
                )
            }
        }
    }

    /// Notify all listeners that the app is now terminated, with `0` being
    /// the status code.
    pub async fn notify_complete(&self) {
        self.submit_result(ExtendedResult::Ok(())).await;

        self.notify.notify_waiters();
    }

    /// Notify all listeners that the app is now terminated, with `0` being
    /// the status code.
    pub async fn notify_with_warnings<T>(&self, warnings: T)
    where
        T: Into<Vec<String>>
    {
        self.submit_result(
            ExtendedResult::Ok(())
            .with_warnings(warnings.into())
        ).await;

        self.notify.notify_waiters();
    }

    /// Notify all listeners that the app is now terminated, with `0` being
    /// the status code.
    pub async fn notify_failure(&self, status: u8, err: AppError) {
        self.submit_result(
            ExtendedResult::Err(status, err)
        ).await;

        self.notify.notify_waiters();
    }
}

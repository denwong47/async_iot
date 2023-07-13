use std::sync::Arc;

use async_trait::async_trait;
use tide::{self, Endpoint};

use async_iot_models::{
    system_state::SystemState,
    traits::{HasCachedState, HasState, ResultToOption},
};

use crate::{app::AppState, error::AppError, traits::ResultToJson};

pub struct SystemStateHook {
    app_state: Arc<AppState>,
    state: Arc<SystemState>,
    subset: bool,
}

impl SystemStateHook {
    /// Create a new [`SystemStateHook`] from a `'static` [`SystemState`] situated
    /// behind a [`RwLock`].
    ///
    /// Due to the lifetime, [`lazy_static`] is needed to create the instance.
    pub fn new(app_state: Arc<AppState>, state: Arc<SystemState>, subset: bool) -> Self {
        Self {
            app_state,
            state,
            subset,
        }
    }
}

#[async_trait]
impl<State> Endpoint<State> for SystemStateHook
where
    State: Clone + Send + Sync + 'static,
{
    async fn call(&self, req: tide::Request<State>) -> tide::Result {
        // TODO Refactor this to not hard code the path
        drop(self.app_state.log_visit("/system", req.remote()));

        let subset = if self.subset {
            req.param("subset")
                .map(|key| vec![key])
                .to_option()
                .and_then(|subset| {
                    if subset.get(0).unwrap_or(&"").len() == 0 {
                        // Treat `/system/` as `/system`
                        None
                    } else {
                        Some(subset)
                    }
                })
        } else {
            None
        };

        let target_keys = if self.subset {
            subset
                .and_then(|v| if &v == &[""] { None } else { Some(v) })
                .unwrap_or_else(|| self.state.available_keys())
        } else {
            self.state.available_keys()
        };

        Ok(self
            .state
            .get_cache_or_update(&target_keys)
            .await
            .map_err(|err| tide::Error::new(500, AppError::from(err)))
            .to_tide_response(&target_keys))
    }
}

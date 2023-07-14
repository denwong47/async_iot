use std::sync::RwLock;

use crate::{logger, results, LocalError};
use async_trait::async_trait;

/// Trait for any objects that have a JSON-representable state.
///
/// The state should also support partial queries. A slice of keys in &[`str`] can be passed
/// to retrieve a subset of the state.
#[async_trait]
pub trait HasState {
    /// Get a subset of a state matching the supplied keys.
    ///
    /// This method returns a [`Result`], which will be rendered as a [`results::ResultJson`] by [`get()`].
    async fn try_get(&self, keys: &[&str]) -> Result<results::ResultJson, LocalError>;

    /// Get a full state with all the available keys.
    ///
    /// This method returns a [`Result`], which will be rendered as a [`results::ResultJson`] by [`get()`].
    async fn try_all(&self) -> Result<results::ResultJson, LocalError>;

    /// Get all the available keys for [`get()`] and [`try_get()`].
    fn available_keys(&self) -> Vec<&str>;

    /// Get a subset of a state matching the supplied keys.
    async fn get(&self, keys: &[&str]) -> results::ResultJson {
        match self.try_get(keys).await {
            Ok(json) => json,
            Err(err) => results::ResultJson::from_err(keys, err),
        }
    }

    /// Get the entire state with all the available keys.
    async fn all(&self) -> results::ResultJson {
        match self.try_all().await {
            Ok(json) => json,
            Err(err) => results::ResultJson::from_err(&self.available_keys(), err),
        }
    }
}

#[async_trait]
pub trait HasCachedState: HasState {
    /// Get the reference to the locked cache of this state.
    async fn locked_cache<'a>(&'a self) -> &'a RwLock<Option<results::ResultJson>>;

    /// Put the last available data into the cache.
    async fn put_cache(&self, cache: results::ResultJson) -> Result<(), LocalError> {
        self.locked_cache()
            .await
            .write()
            .map(|mut lock_ref| {
                *lock_ref = Some(cache);
            })
            .map_err(|_| LocalError::LockPoisoned("state cache"))
    }

    /// Get the currently cached data, filtered using the supplied keys.
    async fn get_cache(&self, keys: &[&str]) -> Result<Option<results::ResultJson>, LocalError> {
        self.locked_cache()
            .await
            .read()
            .map(|lock_ref| lock_ref.as_ref().map(|json| json.get(keys)))
            .map_err(|_| LocalError::LockPoisoned("state cache"))
    }

    /// Get the whole cache without filtering.
    ///
    /// # Note
    ///
    /// This method clones the data.
    async fn all_cache(&self) -> Result<Option<results::ResultJson>, LocalError> {
        self.locked_cache()
            .await
            .read()
            .map(|lock_ref| lock_ref.as_ref().map(results::ResultJson::clone))
            .map_err(|_| LocalError::LockPoisoned("state cache"))
    }

    /// Update the cached state.
    ///
    /// # Note
    ///
    /// This method does not require a mutable reference; the cache is behind a [`RwLock`].
    async fn update(&self) -> Result<(), LocalError> {
        match self.try_all().await {
            Ok(json) => self.put_cache(json).await,
            Err(err) => Err(err),
        }
    }

    /// Get the cached version of the filtered data if available.
    ///
    /// Otherwise, update the data then return it, filtered by ``keys``.
    async fn get_cache_or_update(&self, keys: &[&str]) -> Result<results::ResultJson, LocalError> {
        match self.get_cache(keys).await {
            Ok(Some(cache)) => Ok(cache),
            _ => {
                let json_result = self.try_all().await;
                match json_result {
                    Ok(json) => {
                        let subset = json.get(keys);
                        if let Err(err) = self.put_cache(json).await {
                            logger::error(&format!("Could not update cached state due to: {err}"))
                        }

                        Ok(subset)
                    }
                    Err(err) => Err(err),
                }
            }
        }
    }

    /// Get the cached version of the full data if available.
    ///
    /// Otherwise, update the data then return it.
    async fn all_cache_or_update(&self) -> Result<results::ResultJson, LocalError> {
        self.get_cache_or_update(&self.available_keys()).await
    }
}

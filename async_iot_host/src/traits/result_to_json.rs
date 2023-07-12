use async_iot_models::results::ResultJson;
use http_types::mime;
use tide;

use crate::error::AppError;

/// A private trait to convert a [`Result<ResultJson, E>`] into a [`ResultJson`]
/// by populating the `_result` field of the returned JSON.
pub trait ResultToJson {
    fn to_result_json(self, keys: &[&str]) -> ResultJson;

    fn to_tide_response(self, keys: &[&str]) -> tide::Response;
}

impl ResultToJson for Result<ResultJson, tide::Error> {
    /// Create a [`ResultJson`] from a [`Result<ResultJson, tide::Error>`].
    fn to_result_json(self, keys: &[&str]) -> ResultJson {
        self.unwrap_or_else(|err| ResultJson::from_err(AppError::TideError(err), keys))
    }

    fn to_tide_response(self, keys: &[&str]) -> tide::Response {
        let result = self.and_then(|json| tide::Body::from_json(&json));

        let status_code = if let Err(err) = &result {
            err.status()
        } else {
            tide::StatusCode::Ok
        };

        tide::Response::builder(status_code)
            .body(result.unwrap_or_else(|err| {
                tide::Body::from_json(&ResultJson::from_err(AppError::TideError(err), keys))
                    .unwrap()
            }))
            .content_type(mime::JSON)
            .build()
    }
}

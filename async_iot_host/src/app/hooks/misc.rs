use http_types::mime;
use tide::{self, prelude::*};

use async_iot_models::{logger, results};

/// Return the info page.
pub async fn info(req: tide::Request<()>) -> tide::Result {
    match req.remote() {
        Some(remote) => logger::info(&format!("Rendering information page for '{remote}'.")),
        None => logger::info("Rendering information page for unknown remote."),
    }

    let body = tide::Body::from_json(&results::ResultJson::with_capacity(1).add_result(
        &"info",
        results::ResultState::Ok,
        json!({}),
    ))?;

    Ok(tide::Response::builder(200)
        .body(body)
        .content_type(mime::JSON)
        .build())
}

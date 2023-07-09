use http_types::mime;
use tide::{self, prelude::*};

use async_iot_models::logger;

/// Return the info page.
pub async fn info(req: tide::Request<()>) -> tide::Result {
    match req.remote() {
        Some(remote) => logger::info(&format!("Rendering information page for '{remote}'.")),
        None => logger::info("Rendering information page for unknown remote.")
    }

    let body = tide::Body::from_json(
        &json!(
            {
                "_result": {
                    "info": {
                        "status": "ok",
                    }
                },
                "info": {},
            }
        )
    )?;

    Ok(tide::Response::builder(200)
        .body(body)
        .content_type(mime::JSON)
        .build()
    )
}

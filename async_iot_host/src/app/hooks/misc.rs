use tide;

/// Return the info page.
pub async fn info(_req: tide::Request<()>) -> tide::Result<String> {
    println!("Rendering information page.");
    Ok(String::from("Information page"))
}

/// Perform a get request with `http_req` and returning a string response.
pub fn httpreq_get(url: &str) -> Result<Vec<u8>, http_req::error::Error> {
    let mut buffer = Vec::new();
    http_req::request::get(url, &mut buffer)?;
    Ok(buffer)
}

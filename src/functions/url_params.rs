/// Join multiple url params together into a single string
pub fn url_params(id: Option<&str>, params: Option<&str>) -> String {
    match params {
        Some(params) => format!("{}?{}", id.unwrap_or(""), params),
        None => id.unwrap_or("").to_string(),
    }
}

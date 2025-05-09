/// Join multiple url params together into a single string
pub fn url_params(params: Option<&str>) -> String {
    match params {
        Some(params) => format!("?{}", params),
        None => "".to_string(),
    }
}

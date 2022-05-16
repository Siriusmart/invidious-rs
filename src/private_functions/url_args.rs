pub fn url_args(id: Option<&str>, args: Option<&str>) -> String {
    match args {
        Some(args) => format!("{}?{}", id.unwrap_or(""), args),
        None => id.unwrap_or("").to_string(),
    }
}
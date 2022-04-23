#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Caption {
    pub label: String,
    pub languageCode: Option<String>,
    pub url: String,
}

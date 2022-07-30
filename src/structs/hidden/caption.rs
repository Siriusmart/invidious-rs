use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Caption {
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub language: String,
    pub url: String,
}

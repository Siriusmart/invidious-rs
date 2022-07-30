use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FormatStream {
    pub url: String,
    pub itag: String,
    pub r#type: String,
    pub quality: String,
    pub container: String,
    pub encoding: String,
    #[serde(rename(serialize = "qualityLabel", deserialize = "qualityLabel"))]
    pub quality_label: String,
    pub resolution: String,
    pub size: String,
}

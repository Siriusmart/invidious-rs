use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdaptiveFormat {
    #[serde(default)]
    pub index: String,
    #[serde(default)]
    pub bitrate: String,
    #[serde(default)]
    pub init: String,
    pub url: String,
    pub itag: String,
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub clen: String,
    #[serde(default)]
    pub lmt: String,
    #[serde(default)]
    #[serde(rename = "protectionType")]
    pub projection_type: String,
    #[serde(default)]
    pub container: String,
    #[serde(default)]
    pub encoding: String,
    #[serde(default)]
    #[serde(rename = "qualityLabel")]
    pub quality: String,
    #[serde(default)]
    pub resolution: String,
}

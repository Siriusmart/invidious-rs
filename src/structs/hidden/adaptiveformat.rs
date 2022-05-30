#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdaptiveFormat {
    pub index: Option<String>,
    pub bitrate: String,
    pub init: Option<String>,
    pub url: String,
    pub itag: String,
    pub r#type: String,
    pub clen: String,
    pub lmt: String,
    pub projectionType: String,
    pub container: Option<String>,
    pub encoding: Option<String>,
    pub qualityLabel: Option<String>,
    pub resolution: Option<String>,
}

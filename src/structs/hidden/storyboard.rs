use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Storyboard {
    pub url: String,
    #[serde(rename(serialize = "templateUrl", deserialize = "templateUrl"))]
    pub template_url: String,
    pub width: u32,
    pub height: u32,
    pub count: u32,
    pub interval: u32,
    #[serde(rename(serialize = "storyboardWidth", deserialize = "storyboardWidth"))]
    pub storyboard_width: u16,
    #[serde(rename(serialize = "storyboardHeight", deserialize = "storyboardHeight"))]
    pub storyboard_height: u16,
    #[serde(rename(serialize = "storyboardCount", deserialize = "storyboardCount"))]
    pub storyboard_count: u16,
}

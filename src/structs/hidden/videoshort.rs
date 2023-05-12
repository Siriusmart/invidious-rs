use crate::structs::hidden::VideoThumbnail;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoShort {
    #[serde(rename = "videoId")]
    pub id: String,
    pub title: String,
    #[serde(rename = "videoThumbnails")]
    pub thumbnails: Vec<VideoThumbnail>,
    pub author: String,
    #[serde(rename = "lengthSeconds")]
    pub length: u32,
    #[serde(rename = "viewCountText")]
    pub views_text: String,
}

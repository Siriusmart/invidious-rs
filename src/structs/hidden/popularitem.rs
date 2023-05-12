use crate::structs::hidden::VideoThumbnail;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PopularItem {
    pub r#type: String,
    pub title: String,
    #[serde(rename = "videoId")]
    pub id: String,
    #[serde(rename = "videoThumbnails")]
    pub thumbnails: Vec<VideoThumbnail>,

    #[serde(rename = "lengthSeconds")]
    pub length: u32,
    #[serde(rename = "viewCount")]
    pub views: u64,

    pub author: String,
    #[serde(rename = "authorId")]
    pub author_id: String,
    #[serde(rename = "authorUrl")]
    pub author_url: String,

    pub published: u64,
    #[serde(rename = "publishedText")]
    pub published_text: String,
}

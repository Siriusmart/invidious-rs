use crate::structs::hidden::VideoThumbnail;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelVideo {
    pub title: String,
    #[serde(rename = "videoId")]
    pub id: String,
    #[serde(rename = "videoThumbnails")]
    pub thumbnails: Vec<VideoThumbnail>,
    pub description: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,

    #[serde(rename = "viewCount")]
    pub view_count: u64,
    pub published: u64,
    #[serde(rename = "publishedText")]
    pub published_text: String,
    #[serde(rename = "lengthSeconds")]
    pub length: u32,

    pub author: String,
    #[serde(rename = "authorId")]
    pub author_id: String,
    #[serde(rename = "authorUrl")]
    pub author_url: String,

    #[serde(rename = "liveNow")]
    pub live: bool,
    pub premium: bool,
    #[serde(rename = "isUpcoming")]
    pub upcoming: bool,
}

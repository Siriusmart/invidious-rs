use crate::structs::hidden::VideoThumbnail;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelVideo {
    pub title: String,
    #[serde(rename(serialize = "videoId", deserialize = "videoId"))]
    pub id: String,
    #[serde(rename(serialize = "videoThumbnails", deserialize = "videoThumbnails"))]
    pub thumbnails: Vec<VideoThumbnail>,
    pub description: String,
    #[serde(rename(serialize = "descriptionHtml", deserialize = "descriptionHtml"))]
    pub description_html: String,

    #[serde(rename(serialize = "viewCount", deserialize = "viewCount"))]
    pub view_count: u64,
    pub published: u64,
    #[serde(rename(serialize = "publishedText", deserialize = "publishedText"))]
    pub published_text: String,
    #[serde(rename(serialize = "lengthSeconds", deserialize = "lengthSeconds"))]
    pub length: u32,

    pub author: String,
    #[serde(rename(serialize = "authorId", deserialize = "authorId"))]
    pub author_id: String,
    #[serde(rename(serialize = "authorUrl", deserialize = "authorUrl"))]
    pub author_url: String,

    #[serde(rename(serialize = "liveNow", deserialize = "liveNow"))]
    pub live: bool,
    pub premium: bool,
    #[serde(rename(serialize = "isUpcoming", deserialize = "isUpcoming"))]
    pub upcoming: bool,
}

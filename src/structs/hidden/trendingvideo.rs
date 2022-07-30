use crate::structs::hidden::VideoThumbnail;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrendingVideo {
    pub title: String,
    #[serde(rename(serialize = "videoId", deserialize = "videoId"))]
    pub id: String,
    #[serde(rename(serialize = "videoThumbnails", deserialize = "videoThumbnails"))]
    pub thumbmails: Vec<VideoThumbnail>,

    #[serde(rename(serialize = "lengthSeconds", deserialize = "lengthSeconds"))]
    pub length: u32,
    #[serde(rename(serialize = "viewCount", deserialize = "viewCount"))]
    pub views: u64,

    pub author: String,
    #[serde(rename(serialize = "authorId", deserialize = "authorId"))]
    pub author_id: String,
    #[serde(rename(serialize = "authorUrl", deserialize = "authorUrl"))]
    pub author_url: String,

    pub published: u64,
    #[serde(rename(serialize = "publishedText", deserialize = "publishedText"))]
    pub published_text: String,
    pub description: String,
    #[serde(rename(serialize = "descriptionHtml", deserialize = "descriptionHtml"))]
    pub description_html: String,

    #[serde(rename(serialize = "liveNow", deserialize = "liveNow"))]
    pub live: bool,
    #[serde(default)]
    pub paid: bool,
    pub premium: bool,
}

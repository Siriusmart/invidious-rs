use crate::structs::hidden::VideoThumbnail;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MixVideo {
    title: String,
    #[serde(rename(serialize = "mixId", deserialize = "mixId"))]
    id: String,
    #[serde(rename(serialize = "audioId", deserialize = "audioId"))]
    audio_id: String,
    author: String,
    #[serde(rename(serialize = "authorUrl", deserialize = "authorUrl"))]
    author_url: String,
    videos: Vec<VideoThumbnail>,
    index: u32,
    #[serde(rename(serialize = "lengthSeconds", deserialize = "lengthSeconds"))]
    length: u32,
}

use crate::structs::hidden::VideoThumbnail;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Playlist {
    pub title: String,
    #[serde(rename = "playlistId")]
    pub id: String,
    #[serde(rename = "playlistThumbnail")]
    pub thumbnail: String,
    pub author: String,
    #[serde(rename = "authorId")]
    pub author_id: String,
    #[serde(rename = "authorUrl")]
    pub author_url: String,
    #[serde(rename = "videoCount")]
    pub video_count: u32,
    pub videos: Vec<PlaylistVideo>,
    #[serde(rename = "authorVerified")]
    pub verified: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlaylistVideo {
    pub title: String,
    #[serde(rename = "videoId")]
    pub id: String,
    #[serde(rename = "lengthSeconds")]
    pub length: u32,
    #[serde(rename = "videoThumbnails")]
    pub thumbnails: Vec<VideoThumbnail>,
}

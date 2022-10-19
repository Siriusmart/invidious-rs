use crate::structs::hidden::VideoThumbnail;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Playlist {
    pub title: String,
    #[serde(rename(serialize = "playlistId", deserialize = "playlistId"))]
    pub id: String,
    #[serde(rename(serialize = "playlistThumbnail", deserialize = "playlistThumbnail"))]
    pub thumbnail: String,
    pub author: String,
    #[serde(rename(serialize = "authorId", deserialize = "authorId"))]
    pub author_id: String,
    #[serde(rename(serialize = "authorUrl", deserialize = "authorUrl"))]
    pub author_url: String,
    #[serde(rename(serialize = "videoCount", deserialize = "videoCount"))]
    pub video_count: u32,
    pub videos: Vec<PlaylistVideo>,
    #[serde(rename(serialize = "authorVerified", deserialize = "authorVerified"))]
    pub verified: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlaylistVideo {
    pub title: String,
    #[serde(rename(serialize = "videoId", deserialize = "videoId"))]
    pub id: String,
    #[serde(rename(serialize = "lengthSeconds", deserialize = "lengthSeconds"))]
    pub length: u32,
    #[serde(rename(serialize = "videoThumbnails", deserialize = "videoThumbnails"))]
    pub thumbnails: Vec<VideoThumbnail>,
}

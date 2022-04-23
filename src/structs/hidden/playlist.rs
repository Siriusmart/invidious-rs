#![allow(non_snake_case)]

use crate::structs::hidden::VideoThumbnail;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Playlist {
    pub title: String,
    pub playlistId: String,
    pub author: String,
    pub authorId: String,
    pub authorUrl: String,
    pub videoCount: u32,
    pub videos: Vec<PlaylistVideo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlaylistVideo {
    pub title: String,
    pub videoId: String,
    pub lengthSeconds: u32,
    pub videoThumbnails: Vec<VideoThumbnail>,
}

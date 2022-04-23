#![allow(non_snake_case)]

use crate::structs::hidden::VideoThumbnail;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlaylistItem {
    pub title: String,
    pub videoId: String,
    pub author: String,
    pub authorId: String,
    pub authorUrl: String,

    pub videoThumbnails: Vec<VideoThumbnail>,
    pub index: u32,
    pub lengthSeconds: u32,
}

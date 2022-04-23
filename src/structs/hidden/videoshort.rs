#![allow(non_snake_case)]

use crate::structs::hidden::VideoThumbnail;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoShort {
    pub videoId: String,
    pub title: String,
    pub videoThumbnails: Vec<VideoThumbnail>,
    pub author: String,
    pub lengthSeconds: u32,
    pub viewCountText: String,
}

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use crate::structs::hidden::VideoThumbnail;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MixVideo {
    title: String,
      videoId: String,
      author: String,
      authorId: String,
      authorUrl: String,
      videos: Vec<VideoThumbnail>,
      index: u32,
      lengthSeconds: u32
}
#![allow(non_snake_case)]

use crate::structs::hidden::VideoThumbnail;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrendingVideo {
    pub title: String,
    pub videoId: String,
    pub videoThumbnails: Vec<VideoThumbnail>,

    pub lengthSeconds: u32,
    pub viewCount: u64,

    pub author: String,
    pub authorId: String,
    pub authorUrl: String,

    pub published: u64,
    pub publishedText: String,
    pub description: String,
    pub descriptionHtml: String,

    pub liveNow: bool,
    pub paid: Option<bool>,
    pub premium: bool,
}

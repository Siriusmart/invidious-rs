#![allow(non_snake_case)]

use crate::structs::hidden::VideoThumbnail;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelVideo {
    pub title: String,
    pub videoId: String,
    pub author: String,
    pub authorId: String,
    pub authorUrl: String,

    pub videoThumbnails: Vec<VideoThumbnail>,

    pub description: String,
    pub descriptionHtml: String,
    pub viewCount: u64,
    pub published: u64,
    pub publishedText: String,
    pub lengthSeconds: u32,
    pub paid: Option<bool>,
    pub premium: bool,
}

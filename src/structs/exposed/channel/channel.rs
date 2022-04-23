#![allow(non_snake_case)]

use crate::{
    structs::hidden::{AuthorBanner, AuthorThumbnail, ChannelVideo, RelatedChannel},
    traits::PublicItems,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Channel {
    pub author: String,
    pub authorId: String,
    pub authorUrl: String,
    pub authorBanners: Vec<AuthorBanner>,
    pub authorThumbnails: Vec<AuthorThumbnail>,

    pub subCount: u32,
    pub totalViews: u64,
    pub joined: u64,

    pub paid: Option<bool>,
    pub autoGenerated: bool,
    pub isFamilyFriendly: bool,
    pub description: String,
    pub descriptionHtml: String,
    pub allowedRegions: Vec<String>,

    pub latestVideos: Vec<ChannelVideo>,

    pub relatedChannels: Vec<RelatedChannel>,
}

impl PublicItems for Channel {
    fn url(server: &str, args: String) -> String {
        format!("{}/api/v1/channels/{}", server, args)
    }
}
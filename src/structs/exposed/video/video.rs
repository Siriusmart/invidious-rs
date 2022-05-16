#![allow(non_snake_case)]

use crate::{
    structs::hidden::{
        AdaptiveFormat, AuthorThumbnail, Caption, FormatStream, VideoShort, VideoThumbnail, CountryCode,
    },
    traits::PublicItems,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Video {
    pub title: String,
    pub videoId: String,
    pub videoThumbnails: Vec<VideoThumbnail>,

    pub description: String,
    pub desceiptionHtml: Option<String>,
    pub published: u64,
    pub publishedText: String,

    pub keywords: Vec<String>,
    pub viewCount: u64,
    pub likeCount: u32,
    pub dislikeCount: u32,

    pub paid: bool,
    pub premium: bool,
    pub isFamilyFriendly: bool,
    pub allowedRegions: Vec<CountryCode>,
    pub genre: String,
    pub genreserverUrl: Option<String>,

    pub author: String,
    pub authorId: String,
    pub authorUrl: String,
    pub authorThumbnails: Vec<AuthorThumbnail>,

    pub subCountText: String,
    pub lengthSeconds: u32,
    pub allowRatings: bool,
    pub rating: f32,
    pub isListed: bool,
    pub liveNow: bool,
    pub isUpcoming: bool,
    pub pemiereTimestamp: Option<u64>,

    pub hlsserverUrl: Option<String>,
    pub adaptiveFormats: Vec<AdaptiveFormat>,
    pub formatStreams: Vec<FormatStream>,

    pub captions: Vec<Caption>,

    pub recommendedVideos: Vec<VideoShort>,
}

impl PublicItems for Video {
    fn url(server: &str, args: String) -> String {
        format!("{}/api/v1/videos/{}", server, args)
    }
}

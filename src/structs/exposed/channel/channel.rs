use crate::{structs::hidden::*, traits::PublicItems, *};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Channel {
    #[serde(rename = "author")]
    pub name: String,
    #[serde(rename = "authorId")]
    pub id: String,
    #[serde(rename = "authorUrl")]
    pub url: String,
    #[serde(rename = "authorBanners")]
    pub banners: Vec<CommonImage>,
    #[serde(rename = "authorThumbnails")]
    pub thumbnails: Vec<CommonImage>,

    #[serde(rename = "subCount")]
    pub subscribers: u32,
    #[serde(rename = "totalViews")]
    pub total_views: u64,
    pub joined: u64,

    #[serde(rename = "autoGenerated")]
    pub auto_generated: bool,
    #[serde(rename = "isFamilyFriendly")]
    pub family_friendly: bool,
    pub description: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,
    #[serde(rename = "allowedRegions")]
    pub allowed_regions: Vec<CountryCode>,

    #[serde(rename = "latestVideos")]
    pub lastest_videos: Vec<CommonVideo>,

    #[serde(rename = "relatedChannels")]
    pub related_channels: Vec<RelatedChannel>,
}

impl PublicItems for Channel {
    fn url(server: &str, args: String) -> String {
        format!("{server}/api/v1/channels/{args}")
    }
}

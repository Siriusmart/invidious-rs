use crate::{
    structs::hidden::{
        AdaptiveFormat, AuthorThumbnail, Caption, CountryCode, FormatStream, Storyboard,
        VideoShort, VideoThumbnail,
    },
    traits::PublicItems,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Video {
    pub title: String,
    #[serde(rename(serialize = "videoId", deserialize = "videoId"))]
    pub id: String,
    #[serde(rename(serialize = "videoThumbnails", deserialize = "videoThumbnails"))]
    pub thumbnails: Vec<VideoThumbnail>,
    pub storyboards: Vec<Storyboard>,
    pub description: String,
    #[serde(rename(serialize = "descriptionHtml", deserialize = "descriptionHtml"))]
    pub description_html: String,
    pub published: u64,
    #[serde(rename(serialize = "publishedText", deserialize = "publishedText"))]
    pub published_text: String,

    pub keywords: Vec<String>,
    #[serde(rename(serialize = "viewCount", deserialize = "viewCount"))]
    pub views: u64,
    #[serde(rename(serialize = "likeCount", deserialize = "likeCount"))]
    pub likes: u32,
    #[serde(rename(serialize = "dislikeCount", deserialize = "dislikeCount"))]
    pub dislikes: u32,

    pub paid: bool,
    pub premium: bool,
    #[serde(rename(serialize = "isFamilyFriendly", deserialize = "isFamilyFriendly"))]
    pub family_friendly: bool,
    #[serde(rename(serialize = "allowedRegions", deserialize = "allowedRegions"))]
    pub allowed_regions: Vec<CountryCode>,
    pub genre: String,
    #[serde(rename(serialize = "genreUrl", deserialize = "genreUrl"))]
    pub genre_url: String,

    pub author: String,
    #[serde(rename(serialize = "authorId", deserialize = "authorId"))]
    pub author_id: String,
    #[serde(rename(serialize = "authorUrl", deserialize = "authorUrl"))]
    pub author_url: String,
    #[serde(rename(serialize = "authorThumbnails", deserialize = "authorThumbnails"))]
    pub author_thumbnails: Vec<AuthorThumbnail>,

    #[serde(rename(serialize = "subCountText", deserialize = "subCountText"))]
    pub sub_count_text: String,
    #[serde(rename(serialize = "lengthSeconds", deserialize = "lengthSeconds"))]
    pub length: u32,
    #[serde(rename(serialize = "allowRatings", deserialize = "allowRatings"))]
    pub allow_ratings: bool,
    pub rating: f32,
    #[serde(rename(serialize = "isListed", deserialize = "isListed"))]
    pub listed: bool,
    #[serde(rename(serialize = "liveNow", deserialize = "liveNow"))]
    pub live: bool,
    #[serde(rename(serialize = "isUpcoming", deserialize = "isUpcoming"))]
    pub upcoming: bool,
    #[serde(rename(serialize = "dashUrl", deserialize = "dashUrl"))]
    pub dash: String,

    #[serde(rename(serialize = "adaptiveFormats", deserialize = "adaptiveFormats"))]
    pub adaptive_formats: Vec<AdaptiveFormat>,
    #[serde(rename(serialize = "formatStreams", deserialize = "formatStreams"))]
    pub format_streams: Vec<FormatStream>,

    pub captions: Vec<Caption>,

    #[serde(rename(serialize = "recommendedVideos", deserialize = "recommendedVideos"))]
    pub recommended_videos: Vec<VideoShort>,
}

impl PublicItems for Video {
    fn url(server: &str, args: String) -> String {
        format!("{server}/api/v1/videos/{args}")
    }
}

use crate::structs::hidden::AuthorThumbnail;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Comment {
    #[serde(rename(serialize = "commentId", deserialize = "commentId"))]
    pub id: String,
    #[serde(rename(serialize = "likeCount", deserialize = "likeCount"))]
    pub likes: u32,
    #[serde(rename(serialize = "isEdited", deserialize = "isEdited"))]
    pub edited: bool,
    pub content: String,
    #[serde(rename(serialize = "contentHtml", deserialize = "contentHtml"))]
    pub content_html: String,
    pub published: u64,
    #[serde(rename(serialize = "publishedText", deserialize = "publishedText"))]
    pub published_text: String,

    pub author: String,
    #[serde(rename(serialize = "authorThumbnails", deserialize = "authorThumbnails"))]
    pub author_thumbnails: Vec<AuthorThumbnail>,
    #[serde(rename(serialize = "authorId", deserialize = "authorId"))]
    pub author_id: String,
    #[serde(rename(serialize = "authorUrl", deserialize = "authorUrl"))]
    pub author_url: String,


    #[serde(rename(serialize = "authorIsChannelOwner", deserialize = "authorIsChannelOwner"))]
    pub channel_owner: bool,
    #[serde(default)]
    #[serde(rename(serialize = "createrHeart", deserialize = "createrHeart"))]
    pub heart: Option<CreatorHeart>,
    #[serde(default)]
    pub replies: Option<Replies>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatorHeart {
    #[serde(rename(serialize = "creatorThumbnail", deserialize = "creatorThumbnail"))]
    pub thumbnail: String,
    #[serde(rename(serialize = "creatorName", deserialize = "creatorName"))]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Replies {
    #[serde(rename(serialize = "replyCount", deserialize = "replyCount"))]
    replies: u32,
    continuation: String,
}

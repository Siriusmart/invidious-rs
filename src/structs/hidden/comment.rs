#![allow(non_snake_case)]

use crate::structs::hidden::AuthorThumbnail;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Comment {
    pub author: String,
    pub authorThumbnails: Vec<AuthorThumbnail>,
    pub authorId: String,
    pub authorUrl: String,
    pub isEdited: bool,
    pub content: String,
    pub contentHtml: String,
    pub published: u64,
    pub publishedText: String,
    pub likeCount: u32,
    pub commentId: String,
    pub authorIsChannelOwner: bool,
    pub creatorHeart: Option<CreatorHeart>,
    pub replies: Option<Replies>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatorHeart {
    pub creatorThumbnail: String,
    pub creatorName: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Replies {
    replyCount: u32,
    continuation: String,
}

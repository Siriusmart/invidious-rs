use crate::structs::hidden::AuthorThumbnail;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthorBanner {
    pub url: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RelatedChannel {
    #[serde(rename(serialize = "author", deserialize = "author"))]
    pub name: String,
    #[serde(rename(serialize = "authorId", deserialize = "authorId"))]
    pub id: String,
    #[serde(rename(serialize = "authorUrl", deserialize = "authorUrl"))]
    pub url: String,
    #[serde(rename(serialize = "authorThumbnails", deserialize = "authorThumbnails"))]
    pub thumbnails: Vec<AuthorThumbnail>,
}

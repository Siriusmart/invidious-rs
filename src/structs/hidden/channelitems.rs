#![allow(non_snake_case)]

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
    pub author: String,
    pub authorId: String,
    pub authorUrl: String,
    pub authorThumbnails: Vec<AuthorThumbnail>,
}

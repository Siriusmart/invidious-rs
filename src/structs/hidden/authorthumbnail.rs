#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthorThumbnail {
    pub url: String,
    pub width: u32,
    pub height: u32,
}

#![allow(non_snake_case)]

use crate::{structs::hidden::Comment, traits::PublicItems};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Comments {
    pub commentCount: Option<u32>,
    pub videoId: String,
    pub comments: Vec<Comment>,
    pub continuation: Option<String>,
}

impl PublicItems for Comments {
    fn url(server: &str, args: String) -> String {
        format!("{}/api/v1/comments/{}", server, args)
    }
}

#![allow(non_snake_case)]

use crate::{structs::hidden::Comment, traits::PublicItems};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelComments {
    pub authorId: String,
    pub comments: Vec<Comment>,
    pub content: Option<String>,
    pub continuation: Option<String>,
}

impl PublicItems for ChannelComments {
    fn url(server: &str, args: String) -> String {
        format!("{}/api/v1/channels/comments/{}", server, args)
    }
}

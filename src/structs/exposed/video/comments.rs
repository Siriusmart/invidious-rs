use crate::{structs::hidden::Comment, traits::PublicItems};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Comments {
    #[serde(rename = "commentCount")]
    pub comment_count: Option<u32>,
    #[serde(rename = "videoId")]
    pub id: String,
    pub comments: Vec<Comment>,
    pub continuation: Option<String>,
}

impl PublicItems for Comments {
    fn url(id: &str, params: &str) -> String {
        format!("api/v1/comments/{id}{params}")
    }
}

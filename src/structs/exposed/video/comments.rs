use crate::{structs::hidden::Comment, traits::PublicItems};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Comments {
    #[serde(rename(serialize = "commentCount", deserialize = "commentCount"))]
    pub comment_count: u32,
    #[serde(rename(serialize = "videoId", deserialize = "videoId"))]
    pub video_id: String,
    pub comments: Vec<Comment>,
    pub continuation: Option<String>,
}

impl PublicItems for Comments {
    fn url(server: &str, args: String) -> String {
        format!("{}/api/v1/comments/{}", server, args)
    }
}

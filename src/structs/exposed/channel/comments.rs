use crate::{structs::hidden::Comment, traits::PublicItems};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelComments {
    #[serde(rename = "authorId")]
    pub author_id: String,
    pub comments: Vec<Comment>,
    #[serde(default)]
    pub content: String,
    pub continuation: Option<String>,
}

impl PublicItems for ChannelComments {
    fn url(args: String) -> String {
        format!("api/v1/channels/{args}/community?",)
    }
}

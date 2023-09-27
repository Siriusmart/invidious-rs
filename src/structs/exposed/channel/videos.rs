use crate::{traits::PublicItems, CommonVideo};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelVideos {
    pub videos: Vec<CommonVideo>,
    pub continuation: Option<String>,
}

impl PublicItems for ChannelVideos {
    fn url(server: &str, args: String) -> String {
        format!("{server}/api/v1/channels/videos/{args}")
    }
}

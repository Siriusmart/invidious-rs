use crate::{traits::PublicItems, CommonPlaylist};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelPlaylists {
    pub playlists: Vec<CommonPlaylist>,
    pub continuation: Option<String>,
}

impl PublicItems for ChannelPlaylists {
    fn url(id: &str, params: &str) -> String {
        format!("api/v1/channels/{id}/playlists{params}")
    }
}

use crate::{structs::hidden::Playlist, traits::PublicItems};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelPlaylists {
    pub playlists: Vec<Playlist>,
    pub continuation: Option<String>,
}

impl PublicItems for ChannelPlaylists {
    fn url(server: &str, args: String) -> String {
        format!("{server}/api/v1/channels/playlists/{args}")
    }
}

#![allow(non_snake_case)]

use crate::{
    structs::hidden::{AuthorThumbnail, PlaylistItem},
    traits::PublicItems,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Playlist {
    pub title: String,
    pub playlistId: String,

    pub author: String,
    pub authorId: String,
    pub authorThumbnails: Vec<AuthorThumbnail>,
    pub description: String,
    pub descriptionHtml: String,

    pub videoCount: u32,
    pub viewCount: u64,
    pub updated: u64,

    pub videos: Vec<PlaylistItem>,
}

impl PublicItems for Playlist {
    fn url(server: &str, args: String) -> String {
        format!("{}/api/v1/playlists/{}", server, args)
    }
}

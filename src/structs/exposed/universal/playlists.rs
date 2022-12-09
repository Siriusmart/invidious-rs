use crate::{
    structs::hidden::{AuthorThumbnail, PlaylistItem},
    traits::PublicItems,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Playlist {
    pub title: String,
    #[serde(rename(serialize = "playlistId", deserialize = "playlistId"))]
    pub id: String,
    #[serde(rename(serialize = "playlistThumbnail", deserialize = "playlistThumbnail"))]
    pub thumbnail: String,

    pub author: String,
    #[serde(rename(serialize = "authorId", deserialize = "authorId"))]
    pub author_id: String,
    #[serde(rename(serialize = "authorThumbnails", deserialize = "authorThumbnails"))]
    pub author_thumbnails: Vec<AuthorThumbnail>,
    pub description: String,
    #[serde(rename(serialize = "descriptionHtml", deserialize = "descriptionHtml"))]
    pub description_html: String,

    #[serde(rename(serialize = "videoCount", deserialize = "videoCount"))]
    pub video_count: u32,
    #[serde(rename(serialize = "viewCount", deserialize = "viewCount"))]
    pub views: u64,
    pub updated: u64,
    #[serde(rename(serialize = "isListed", deserialize = "isListed"))]
    pub listed: bool,

    pub videos: Vec<PlaylistItem>,
}

impl PublicItems for Playlist {
    fn url(server: &str, args: String) -> String {
        format!("{}/api/v1/playlists/{}", server, args)
    }
}

use crate::{
    structs::hidden::{AuthorThumbnail, PlaylistItem},
    traits::PublicItems,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Playlist {
    pub title: String,
    #[serde(rename = "playlistId")]
    pub id: String,
    #[serde(rename = "playlistThumbnail")]
    pub thumbnail: String,

    pub author: String,
    #[serde(rename = "authorId")]
    pub author_id: String,
    #[serde(rename = "authorThumbnails")]
    pub author_thumbnails: Vec<AuthorThumbnail>,
    pub description: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,

    #[serde(rename = "videoCount")]
    pub video_count: u32,
    #[serde(rename = "viewCount")]
    pub views: u64,
    pub updated: u64,
    #[serde(rename = "isListed")]
    pub listed: bool,

    pub videos: Vec<PlaylistItem>,
}

impl PublicItems for Playlist {
    fn url(server: &str, args: String) -> String {
        format!("{}/api/v1/playlists/{}", server, args)
    }

    fn from_value(mut value: serde_json::Value) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized + serde::de::DeserializeOwned,
    {
        if process_value(&mut value).is_none() {
            return Err(crate::errors::InvidiousError::InvalidRequest("You are either missing `playlistThumbnail`, or `playlistThumbnail` is null and `videos[0].videoThumbnails.url` does not exist".to_string()).into());
        }

        Ok(serde_json::from_value(value)?)
    }
}

fn process_value(value: &mut serde_json::Value) -> Option<()> {
    if value.get("playlistThumbnail")?.is_null() {
        *value.get_mut("playlistThumbnail")? = value
            .get("videos")?
            .as_array()?
            .first()?
            .get("videoThumbnails")?
            .as_array()?
            .first()?
            .get("url")?
            .clone()
    }

    Some(())
}

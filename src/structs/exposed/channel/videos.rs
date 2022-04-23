#![allow(non_snake_case)]

use crate::{structs::hidden::ChannelVideo, traits::PublicItems};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelVideos {
    pub videos: Vec<ChannelVideo>,
}

impl PublicItems for ChannelVideos {
    fn url(server: &str, args: String) -> String {
        format!("{}/api/v1/channels/videos/{}", server, args)
    }

    fn from_str<'a>(s: &'a str) -> Result<Self, serde_json::Error>
    where
            Self: Sized + Deserialize<'a>, {
        Ok(Self {
            videos: serde_json::from_str(s)?,
        })
    }
}

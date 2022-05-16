#![allow(non_snake_case)]

use std::error::Error;

use crate::{structs::hidden::ChannelVideo, traits::PublicItems};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelVideos {
    pub videos: Vec<ChannelVideo>,
}

impl PublicItems for ChannelVideos {
    fn url(server: &str, args: String) -> String {
        format!("{}/api/v1/channels/videos/{}", server, args)
    }

    fn from_value(value: Value) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized + DeserializeOwned,  {
        Ok(Self {
            videos: serde_json::from_value(value)?,
        })
    }
}

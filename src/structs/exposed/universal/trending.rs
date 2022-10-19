use std::error::Error;

use crate::{structs::hidden::TrendingVideo, traits::PublicItems};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trending {
    pub videos: Vec<TrendingVideo>,
}

impl PublicItems for Trending {
    fn url(server: &str, args: String) -> String {
        format!("{server}/api/v1/trending/{args}")
    }

    fn from_value<'a>(value: Value) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized + DeserializeOwned,
    {
        Ok(Self {
            videos: serde_json::from_value(value)?,
        })
    }
}

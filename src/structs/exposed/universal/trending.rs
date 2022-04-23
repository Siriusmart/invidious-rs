#![allow(non_snake_case)]

use crate::{structs::hidden::TrendingVideo, traits::PublicItems};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trending {
    pub videos: Vec<TrendingVideo>,
}

impl PublicItems for Trending {
    fn url(server: &str, args: String) -> String {
        format!("{}/api/v1/trending/{}", server, args)
    }

    fn from_str<'a>(s: &'a str) -> Result<Self, serde_json::Error>
    where
        Self: Sized + Deserialize<'a>,
    {
        Ok(Self {
            videos: serde_json::from_str(s)?,
        })
    }
}

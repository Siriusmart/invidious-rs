#![allow(non_snake_case)]

use crate::{structs::hidden::MixVideo, traits::PublicItems};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mix {
    title: String,
    mixId: String,
    videos: Vec<MixVideo>,
}

impl PublicItems for Mix {
    fn url(server: &str, args: String) -> String {
        format!("{}/api/v1/mixes/{}", server, args)
    }
}

use crate::{structs::hidden::MixVideo, traits::PublicItems};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mix {
    pub title: String,
    #[serde(rename = "midId")]
    pub id: String,
    pub videos: Vec<MixVideo>,
}

impl PublicItems for Mix {
    fn url(id: &str, params: &str) -> String {
        format!("api/v1/mixes/{id}{params}")
    }
}

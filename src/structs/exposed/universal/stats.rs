use crate::{
    structs::hidden::{Metadata, Software, Usage},
    traits::PublicItems,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stats {
    pub version: Option<String>,
    pub software: Software,
    pub registrations: Option<bool>,
    #[serde(rename = "openRegistrations")]
    pub usage: Option<Usage>,
    pub metadata: Option<Metadata>,
}

impl PublicItems for Stats {
    fn url(_id: &str, params: &str) -> String {
        format!("api/v1/stats/{params}")
    }
}

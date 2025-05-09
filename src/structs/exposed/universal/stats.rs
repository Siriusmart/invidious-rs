use crate::{
    structs::hidden::{Metadata, Software, Usage},
    traits::PublicItems,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stats {
    pub version: String,
    pub software: Software,
    #[serde(rename = "openRegistrations")]
    pub registrations: bool,
    pub usage: Usage,
    pub metadata: Metadata,
}

impl PublicItems for Stats {
    fn url(_id: &str, params: &str) -> String {
        format!("api/v1/stats/{params}")
    }
}

#![allow(non_snake_case)]

use crate::{
    structs::hidden::{Metadata, Software, Usage},
    traits::PublicItems,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stats {
    pub version: String,
    pub software: Software,
    pub openRegistrations: bool,
    pub usage: Usage,
    pub metadata: Metadata,
}

impl PublicItems for Stats {
    fn url(server: &str, args: String) -> String {
        format!("{}/api/v1/stats/{}", server, args)
    }
}

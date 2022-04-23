#![allow(non_snake_case)]

use crate::{structs::hidden::PopularItem, traits::PublicItems};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Popular {
    pub items: Vec<PopularItem>,
}

impl PublicItems for Popular {
    fn url(server: &str, args: String) -> String {
        format!("{}/api/v1/popular/{}", server, args)
    }

    fn from_str<'a>(s: &'a str) -> Result<Self, serde_json::Error>
    where
        Self: Sized + Deserialize<'a>,
    {
        Ok(Self {
            items: serde_json::from_str(s)?,
        })
    }
}

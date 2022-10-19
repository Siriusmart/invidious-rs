use crate::{structs::hidden::PopularItem, traits::PublicItems};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Popular {
    pub items: Vec<PopularItem>,
}

impl PublicItems for Popular {
    fn url(server: &str, args: String) -> String {
        format!("{server}/api/v1/popular/{args}")
    }

    fn from_value(value: Value) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized + DeserializeOwned,
    {
        Ok(Self {
            items: serde_json::from_value(value)?,
        })
    }
}

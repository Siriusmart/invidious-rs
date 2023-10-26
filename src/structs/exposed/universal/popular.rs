use crate::{structs::hidden::PopularItem, traits::PublicItems, InvidiousError};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Popular {
    pub items: Vec<PopularItem>,
}

impl PublicItems for Popular {
    fn url(args: String) -> String {
        format!("api/v1/popular/{args}")
    }

    fn from_value(value: Value) -> Result<Self, InvidiousError>
    where
        Self: Sized + DeserializeOwned,
    {
        Ok(Self {
            items: InvidiousError::as_serde_error(serde_json::from_value(value), None)?,
        })
    }
}

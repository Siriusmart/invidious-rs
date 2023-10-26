use crate::{structs::hidden::SearchItem, traits::PublicItems, InvidiousError};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelSearch {
    pub items: Vec<SearchItem>,
}

impl PublicItems for ChannelSearch {
    fn url(args: String) -> String {
        format!("api/v1/channels/search/{args}")
    }

    fn from_value(value: Value) -> Result<Self, InvidiousError>
    where
        Self: Sized + DeserializeOwned,
    {
        let items: Vec<SearchItem> =
            InvidiousError::as_serde_error(serde_json::from_value(value), None)?;

        Ok(ChannelSearch { items })
    }
}

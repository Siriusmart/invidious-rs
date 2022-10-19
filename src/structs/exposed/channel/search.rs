use std::error::Error;

use crate::{
    structs::hidden::{SearchItem, SearchItemTransition},
    traits::PublicItems,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelSearch {
    pub items: Vec<SearchItem>,
}

impl PublicItems for ChannelSearch {
    fn url(server: &str, args: String) -> String {
        format!("{}/api/v1/channels/search/{}", server, args)
    }

    fn from_value(value: Value) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized + DeserializeOwned,
    {
        let search_transition: Vec<SearchItemTransition> = serde_json::from_value(value)?;
        let items: Vec<SearchItem> = search_transition
            .into_iter()
            .map(|search_item_transition| search_item_transition.proccess())
            .collect();

        Ok(ChannelSearch { items })
    }
}

#![allow(non_snake_case)]

use crate::{
    structs::hidden::{SearchItem, SearchItemTransition},
    traits::PublicItems,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelSearch {
    pub items: Vec<SearchItem>,
}

impl PublicItems for ChannelSearch {
    fn url(server: &str, args: String) -> String {
        format!("{}/api/v1/channels/search/{}", server, args)
    }

    fn from_str<'a>(s: &'a str) -> Result<Self, serde_json::Error>
    where
        Self: Sized + Deserialize<'a>,
    {
        let search_transition: Vec<SearchItemTransition> = serde_json::from_str(s)?;
        let items: Vec<SearchItem> = search_transition
            .into_iter()
            .map(|search_item_transition| search_item_transition.proccess())
            .collect();

        Ok(ChannelSearch { items })
    }
}

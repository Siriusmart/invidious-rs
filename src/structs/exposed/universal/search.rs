use crate::{
    structs::hidden::{SearchItem, SearchItemTransition},
    traits::PublicItems,
    InvidiousError,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Search {
    pub items: Vec<SearchItem>,
}

impl PublicItems for Search {
    fn url(server: &str, args: String) -> String {
        format!("{}/api/v1/search/{}", server, args)
    }

    fn from_value(value: Value) -> Result<Self, InvidiousError>
    where
        Self: Sized + DeserializeOwned,
    {
        let search_transition: Vec<SearchItemTransition> =
            InvidiousError::as_serde_error(serde_json::from_value(value), None)?;
        let items: Vec<SearchItem> = search_transition
            .into_iter()
            .map(|search_item_transition| search_item_transition.proccess())
            .collect();

        Ok(Search { items })
    }
}

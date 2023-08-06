use crate::{traits::PublicItems, CommonVideo, InvidiousError};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trending {
    pub videos: Vec<CommonVideo>,
}

impl PublicItems for Trending {
    fn url(server: &str, args: String) -> String {
        format!("{server}/api/v1/trending/{args}")
    }

    fn from_value<'a>(value: Value) -> Result<Self, InvidiousError>
    where
        Self: Sized + DeserializeOwned,
    {
        Ok(Self {
            videos: InvidiousError::as_serde_error(serde_json::from_value(value), None)?,
        })
    }
}

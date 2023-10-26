use crate::{traits::PublicItems, InvidiousError};

pub type Annotations = String;

impl PublicItems for Annotations {
    fn url(args: String) -> String {
        format!("api/v1/annotations/{args}")
    }

    fn from_str<'a>(s: String) -> Result<Self, InvidiousError>
    where
        Self: Sized + serde::de::DeserializeOwned,
    {
        Ok(s)
    }

    fn to_string<'a>(&self) -> String
    where
        Self: serde::Serialize + serde::Deserialize<'a> + std::fmt::Debug,
    {
        self.clone()
    }

    // fn from_value<'a>(value: serde_json::Value) -> Result<Self, InvidiousError>
    // where
    //     Self: Sized + serde::de::DeserializeOwned,
    // {
    //     Ok(serde_json::to_string(&value)?)
    // }
}

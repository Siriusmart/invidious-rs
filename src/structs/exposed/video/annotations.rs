use crate::traits::PublicItems;

pub type Annotations = String;

impl PublicItems for Annotations {
    fn url(server: &str, args: String) -> String {
        format!("{server}/api/v1/annotations/{args}")
    }

    fn from_str<'a>(s: &'a str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized + serde::de::DeserializeOwned,
    {
        Ok(s.to_string())
    }

    fn to_string<'a>(&self) -> String
    where
        Self: serde::Serialize + serde::Deserialize<'a> + std::fmt::Debug,
    {
        self.clone()
    }

    fn from_value<'a>(value: serde_json::Value) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized + serde::de::DeserializeOwned,
    {
        Ok(serde_json::to_string(&value)?)
    }
}

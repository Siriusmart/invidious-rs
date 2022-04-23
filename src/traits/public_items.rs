use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub trait PublicItems {
    fn from_str<'a>(s: &'a str) -> Result<Self, serde_json::Error>
    where
        Self: Sized + Deserialize<'a>,
    {
        Ok(serde_json::from_str(s)?)
    }

    fn to_string<'a>(&self) -> String
    where
        Self: Serialize + Deserialize<'a> + Debug,
    {
        serde_json::to_string(self).unwrap()
    }

    fn url(server: &str, args: String) -> String;
}

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use std::{error::Error, fmt::Debug};

use crate::errors::InvidiousError;

pub trait PublicItems {
    // Error gate keeping
    fn from_str<'a>(s: &'a str) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized + DeserializeOwned,
    {
        let value: Value = serde_json::from_str(s)?;

        match &value["error"] {
            Value::String(s) => Err(Box::new(InvidiousError::InvalidRequest(s.clone()))),
            _ => Ok(Self::from_value(value)?),
        }
    }

    fn from_value<'a>(value: Value) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized + DeserializeOwned,
    {
        Ok(serde_json::from_value(value)?)
    }

    fn to_string<'a>(&self) -> String
    where
        Self: Serialize + Deserialize<'a> + Debug,
    {
        serde_json::to_string(self).unwrap()
    }

    fn url(server: &str, args: String) -> String;
}

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;

#[cfg(any(feature = "sync", feature = "async"))]
use super::*;
use crate::errors::*;
#[cfg(any(feature = "sync", feature = "async"))]
use crate::functions::*;

/// A trait which contains all basic functionalites a binding struct should include.
#[cfg_attr(feature = "async", async_trait::async_trait)]
pub trait PublicItems {
    // Error gate keeping
    /// Converts &str to Self.
    fn from_str(s: String) -> Result<Self, InvidiousError>
    where
        Self: Sized + DeserializeOwned,
    {
        let value: Value = InvidiousError::as_serde_error(serde_json::from_str(&s), Some(s))?;
        let error = value["error"].as_str().map(str::to_string);
        match Self::from_value(value) {
            Ok(val) => Ok(val),
            Err(e) => match error {
                Some(s) => Err(InvidiousError::ApiError { message: s }),
                None => Err(e),
            },
        }
    }

    /// Blocking function to fetch the json data and deserialising it into Self.
    #[cfg(feature = "sync")]
    fn fetch_sync<C: ClientSyncTrait>(
        client: &C,
        id: Option<&str>,
        params: Option<&str>,
    ) -> Result<Self, InvidiousError>
    where
        Self: Sized + DeserializeOwned,
    {
        let url = Self::url(id.unwrap_or_default(), &url_params(params));
        let res = InvidiousError::as_fetch_error(client.fetch(&url))?;
        Self::from_str(res)
    }

    /// Async function to fetch the json data and deserialising it into Self.
    #[cfg(feature = "async")]
    async fn fetch_async<C: ClientAsyncTrait>(
        client: &C,
        id: Option<&str>,
        params: Option<&str>,
    ) -> Result<Self, InvidiousError>
    where
        Self: Sized + DeserializeOwned,
    {
        let url = Self::url(id.unwrap_or_default(), &url_params(params));
        let res = InvidiousError::as_fetch_error(client.fetch(&url).await)?;
        Self::from_str(res)
    }

    /// Converts Value to Self.
    fn from_value(value: Value) -> Result<Self, InvidiousError>
    where
        Self: Sized + DeserializeOwned,
    {
        InvidiousError::as_serde_error(serde_json::from_value(value), None)
    }

    /// Converts Self to a string.
    fn to_string<'a>(&self) -> String
    where
        Self: Serialize + Deserialize<'a> + Debug,
    {
        serde_json::to_string(self).unwrap()
    }

    /// Returns the endpoint url.
    fn url(id: &str, params: &str) -> String;
}

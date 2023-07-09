use std::{error::Error, fmt::Display};

use serde::de::DeserializeOwned;

/// Error enum for the library.
///
/// With a very depressing one variant...
#[derive(Debug)]
pub enum InvidiousError {
    Fetch {
        error: Box<dyn Error>,
    },
    ApiError {
        message: String,
    },
    SerdeError {
        error: serde_json::Error,
        original: Option<String>,
    },
    Message {
        message: String,
    }, // InvalidRequest(String),
}

impl Display for InvidiousError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fetch { error } => write!(f, "Error sending request: {error}"),
            Self::ApiError { message } => write!(f, "API returned errer: {message}"),
            Self::SerdeError { error, .. } => write!(f, "Cannot deserialize response: {error}"),
            Self::Message { message } => write!(f, "Error message: {message}"),
        }
    }
}

impl Error for InvidiousError {}

impl InvidiousError {
    pub fn as_fetch_error(res: Result<String, Box<dyn Error>>) -> Result<String, Self> {
        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(Self::Fetch { error: e }),
        }
    }

    pub fn as_serde_error<V: DeserializeOwned>(
        res: Result<V, serde_json::Error>,
        original: Option<String>,
    ) -> Result<V, Self> {
        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(Self::SerdeError { error: e, original }),
        }
    }

    pub fn as_api_error(e: Option<&str>) -> Result<(), Self> {
        match e {
            Some(e) => Err(Self::ApiError {
                message: e.to_string(),
            }),
            None => Ok(()),
        }
    }

    pub fn as_message(message: String) -> Self {
        Self::Message { message }
    }
}

use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum InvidiousError {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    InvalidResponse(String),
}

impl Display for InvidiousError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvidiousError::Reqwest(e) => write!(f, "Reqwest error: {}", e),
            InvidiousError::Serde(e) => write!(f, "Serde error: {}", e),
            InvidiousError::InvalidResponse(e) => write!(f, "Invalid response: {}", e),
        }
    }
}

impl Error for InvidiousError {}

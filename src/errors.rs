use std::{error::Error, fmt::Display};

/// Error enum for the library.
/// 
/// With a very depressing one variant...
#[derive(Debug)]
pub enum InvidiousError {
    InvalidRequest(String),
}

impl Display for InvidiousError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvidiousError::InvalidRequest(s) => write!(f, "Invalid request: {}", s),
        }
    }
}

impl Error for InvidiousError {}

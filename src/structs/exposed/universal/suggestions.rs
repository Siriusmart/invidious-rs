use crate::traits::PublicItems;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Suggestions {
    pub query: String,
    pub suggestions: Vec<String>,
}

impl PublicItems for Suggestions {
    fn url(_id: &str, params: &str) -> String {
        format!("api/v1/search/suggestions/{params}")
    }
}

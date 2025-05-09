use crate::{structs::hidden::Caption, traits::PublicItems};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Captions {
    pub captions: Vec<Caption>,
}

impl PublicItems for Captions {
    fn url(id: &str, params: &str) -> String {
        format!("api/v1/captions/{id}{params}")
    }
}

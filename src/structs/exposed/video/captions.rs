#![allow(non_snake_case)]

use crate::{structs::hidden::Caption, traits::PublicItems};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Captions {
    pub captions: Vec<Caption>,
}

impl PublicItems for Captions {
    fn url(server: &str, args: String) -> String {
        format!("{}/api/v1/captions/{}", server, args)
    }
}

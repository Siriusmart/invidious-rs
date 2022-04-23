#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Software {
    pub name: String,
    pub version: String,
    pub branch: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usage {
    pub users: Users,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Users {
    pub total: u32,
    pub activeHalfyear: u32,
    pub activeMonth: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    pub updatedAt: u64,
    pub lastChannelRefreshedAt: u64,
}

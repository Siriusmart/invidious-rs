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
    #[serde(rename(serialize = "activeHalfyear", deserialize = "activeHalfyear"))]
    pub half_year: u32,
    #[serde(rename(serialize = "activeMonth", deserialize = "activeMonth"))]
    pub month: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    #[serde(rename(serialize = "updatedAt", deserialize = "updatedAt"))]
    pub updated: u64,
    #[serde(rename(
        serialize = "lastChannelRefreshedAt",
        deserialize = "lastChannelRefreshedAt"
    ))]
    pub last_channel_refresh: u64,
}

#![allow(non_snake_case)]

use crate::structs::hidden::{AuthorThumbnail, VideoThumbnail};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchItemTransition {
    pub r#type: String,
    pub title: Option<String>,
    pub videoId: Option<String>,
    pub author: String,
    pub authorId: String,
    pub authorUrl: String,
    pub videoThumbnails: Option<Vec<VideoThumbnail>>,
    pub description: Option<String>,
    pub descriptionHtml: Option<String>,
    pub viewCount: Option<u64>,
    pub published: Option<u64>,
    pub publishedText: Option<String>,
    pub lengthSeconds: Option<u64>,
    pub liveNow: Option<bool>,
    pub paid: Option<bool>,
    pub premium: Option<bool>,

    pub playlistId: Option<String>,
    pub videoCount: Option<u32>,
    pub videos: Option<Vec<SearchPlaylistVideo>>,

    pub authorThumbnails: Option<Vec<AuthorThumbnail>>,
    pub subCount: Option<u32>,
}

impl SearchItemTransition {
    pub fn proccess(self) -> SearchItem {
        match self.r#type.as_str() {
            "video" => SearchItem::Video {
                title: self.title.unwrap_or(String::new()),
                videoId: self.videoId.unwrap_or(String::new()),
                lengthSeconds: self.lengthSeconds.unwrap_or(0),
                videoThumbnails: self.videoThumbnails.unwrap_or(Vec::new()),
                description: self.description.unwrap_or(String::new()),
                descriptionHtml: self.descriptionHtml.unwrap_or(String::new()),
                viewCount: self.viewCount.unwrap_or(0),
                published: self.published.unwrap_or(0),
                publishedText: self.publishedText.unwrap_or(String::new()),
                liveNow: self.liveNow.unwrap_or(false),
                paid: self.paid.unwrap_or(false),
                premium: self.premium.unwrap_or(false),
                author: self.author,
                authorId: self.authorId,
                authorUrl: self.authorUrl,
            },
            "playlist" => SearchItem::Playlist {
                title: self.title.unwrap_or(String::new()),
                playlistId: self.playlistId.unwrap_or(String::new()),
                author: self.author,
                authorId: self.authorId,
                authorUrl: self.authorUrl,
                videoCount: self.videoCount.unwrap_or(0),
                videos: self.videos.unwrap_or(Vec::new()),
            },
            "channel" => SearchItem::Channel {
                author: self.author,
                authorId: self.authorId,
                authorUrl: self.authorUrl,

                authorThumbnails: self.authorThumbnails.unwrap_or(Vec::new()),
                subCount: self.subCount.unwrap_or(0),
                videoCount: self.videoCount.unwrap_or(0),
                description: self.description.unwrap_or(String::new()),
                descriptionHtml: self.descriptionHtml.unwrap_or(String::new()),
            },
            _ => SearchItem::Unknown(self),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SearchItem {
    Video {
        title: String,
        videoId: String,
        author: String,
        authorId: String,
        authorUrl: String,
        lengthSeconds: u64,
        videoThumbnails: Vec<VideoThumbnail>,
        description: String,
        descriptionHtml: String,
        viewCount: u64,
        published: u64,
        publishedText: String,
        liveNow: bool,
        paid: bool,
        premium: bool,
    },

    Playlist {
        title: String,
        playlistId: String,
        author: String,
        authorId: String,
        authorUrl: String,
        videoCount: u32,
        videos: Vec<SearchPlaylistVideo>,
    },

    Channel {
        author: String,
        authorId: String,
        authorUrl: String,

        authorThumbnails: Vec<AuthorThumbnail>,
        subCount: u32,
        videoCount: u32,
        description: String,
        descriptionHtml: String,
    },

    Unknown(SearchItemTransition),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchPlaylistVideo {
    pub title: String,
    pub videoId: String,
    pub lengthSeconds: u32,
    pub videoThumbnails: Vec<VideoThumbnail>,
}

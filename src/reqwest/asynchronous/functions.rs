use std::error::Error;

use crate::{
    private_functions::url_args,
    structs::{
        channel::{Channel, ChannelComments, ChannelPlaylists, ChannelSearch, ChannelVideos},
        universal::{Mix, Playlist, Popular, Search, Stats, Trending},
        video::{Annotations, Captions, Comments, Video},
    },
    traits::PublicItems,
};

pub async fn stats(server: &str, args: Option<&str>) -> Result<Stats, Box<dyn Error>> {
    let body = reqwest::get(Stats::url(server, url_args(None, args)))
        .await?
        .text()
        .await?;
    let stats = Stats::from_str(&body)?;
    Ok(stats)
}

pub async fn video(
    server: &str,
    video_id: &str,
    args: Option<&str>,
) -> Result<Video, Box<dyn Error>> {
    let body = reqwest::get(Video::url(server, url_args(Some(video_id), args)))
        .await?
        .text()
        .await?;
    let video = Video::from_str(&body)?;
    Ok(video)
}

pub async fn comments(
    server: &str,
    video_id: &str,
    args: Option<&str>,
) -> Result<Comments, Box<dyn Error>> {
    let body = reqwest::get(Comments::url(server, url_args(Some(video_id), args)))
        .await?
        .text()
        .await?;
    let comments = Comments::from_str(&body)?;
    Ok(comments)
}

pub async fn captions(
    server: &str,
    video_id: &str,
    args: Option<&str>,
) -> Result<Captions, Box<dyn Error>> {
    let body = reqwest::get(Captions::url(server, url_args(Some(video_id), args)))
        .await?
        .text()
        .await?;
    let captions = Captions::from_str(&body)?;
    Ok(captions)
}

pub async fn annotations(
    server: &str,
    video_id: &str,
    args: Option<&str>,
) -> Result<Annotations, Box<dyn Error>> {
    let body = reqwest::get(Annotations::url(server, url_args(Some(video_id), args)))
        .await?
        .text()
        .await?;
    let annotations = Annotations::from_str(&body).unwrap();
    Ok(annotations)
}

pub async fn trending(server: &str, args: Option<&str>) -> Result<Trending, Box<dyn Error>> {
    let body = reqwest::get(Trending::url(server, url_args(None, args)))
        .await?
        .text()
        .await?;
    let videos = Trending::from_str(&body)?;
    Ok(videos)
}

pub async fn popular(server: &str, args: Option<&str>) -> Result<Popular, Box<dyn Error>> {
    let body = reqwest::get(Popular::url(server, url_args(None, args)))
        .await?
        .text()
        .await?;
    let videos = Popular::from_str(&body)?;
    Ok(videos)
}

pub async fn channel(
    server: &str,
    id: &str,
    args: Option<&str>,
) -> Result<Channel, Box<dyn Error>> {
    let body = reqwest::get(Channel::url(server, url_args(Some(id), args)))
        .await?
        .text()
        .await?;
    let channel = Channel::from_str(&body)?;
    Ok(channel)
}

pub async fn channel_videos(
    server: &str,
    id: &str,
    args: Option<&str>,
) -> Result<ChannelVideos, Box<dyn Error>> {
    let body = reqwest::get(ChannelVideos::url(server, url_args(Some(id), args)))
        .await?
        .text()
        .await?;
    let videos = ChannelVideos::from_str(&body)?;
    Ok(videos)
}

pub async fn channel_playlists(
    server: &str,
    id: &str,
    args: Option<&str>,
) -> Result<ChannelPlaylists, Box<dyn Error>> {
    let body = reqwest::get(ChannelPlaylists::url(server, url_args(Some(id), args)))
        .await?
        .text()
        .await?;
    let playlists = ChannelPlaylists::from_str(&body)?;
    Ok(playlists)
}

pub async fn channel_comments(
    server: &str,
    id: &str,
    args: Option<&str>,
) -> Result<ChannelComments, Box<dyn Error>> {
    let body = reqwest::get(ChannelComments::url(server, url_args(Some(id), args)))
        .await?
        .text()
        .await?;
    let comments = ChannelComments::from_str(&body)?;
    Ok(comments)
}

pub async fn channel_search(
    server: &str,
    id: &str,
    args: Option<&str>,
) -> Result<ChannelSearch, Box<dyn Error>> {
    let body = reqwest::get(ChannelSearch::url(server, url_args(Some(id), args)))
        .await?
        .text()
        .await?;
    let search = ChannelSearch::from_str(&body)?;

    Ok(search)
}

pub async fn search(server: &str, args: Option<&str>) -> Result<Search, Box<dyn Error>> {
    let body = reqwest::get(Search::url(server, url_args(None, args)))
        .await?
        .text()
        .await?;
    let search = Search::from_str(&body)?;
    Ok(search)
}

pub async fn playlist(
    server: &str,
    id: &str,
    args: Option<&str>,
) -> Result<Playlist, Box<dyn Error>> {
    let body = reqwest::get(Playlist::url(server, url_args(Some(id), args)))
        .await?
        .text()
        .await?;
    let playlist = Playlist::from_str(&body)?;
    Ok(playlist)
}

pub async fn mix(server: &str, id: &str, args: Option<&str>) -> Result<Mix, Box<dyn Error>> {
    let body = reqwest::get(Mix::url(server, url_args(Some(id), args)))
        .await?
        .text()
        .await?;
    let mix = Mix::from_str(&body)?;
    Ok(mix)
}

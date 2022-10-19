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

pub fn stats(server: &str, args: Option<&str>) -> Result<Stats, Box<dyn Error>> {
    let body = reqwest::blocking::get(Stats::url(server, url_args(None, args)))?.text()?;
    let stats = Stats::from_str(&body)?;
    Ok(stats)
}

pub fn video(server: &str, video_id: &str, args: Option<&str>) -> Result<Video, Box<dyn Error>> {
    let body =
        reqwest::blocking::get(Video::url(server, url_args(Some(video_id), args)))?.text()?;
    let video = Video::from_str(&body)?;
    Ok(video)
}

pub fn comments(
    server: &str,
    video_id: &str,
    args: Option<&str>,
) -> Result<Comments, Box<dyn Error>> {
    let body =
        reqwest::blocking::get(Comments::url(server, url_args(Some(video_id), args)))?.text()?;
    let comments = Comments::from_str(&body)?;
    Ok(comments)
}

pub fn captions(
    server: &str,
    video_id: &str,
    args: Option<&str>,
) -> Result<Captions, Box<dyn Error>> {
    let body =
        reqwest::blocking::get(Captions::url(server, url_args(Some(video_id), args)))?.text()?;
    let captions = Captions::from_str(&body)?;
    Ok(captions)
}

pub fn annotations(
    server: &str,
    video_id: &str,
    args: Option<&str>,
) -> Result<Annotations, Box<dyn Error>> {
    let body =
        reqwest::blocking::get(Annotations::url(server, url_args(Some(video_id), args)))?.text()?;
    let annotations = Annotations::from_str(&body)?;
    Ok(annotations)
}

pub fn trending(server: &str, args: Option<&str>) -> Result<Trending, Box<dyn Error>> {
    let body = reqwest::blocking::get(Trending::url(server, url_args(None, args)))?.text()?;
    let videos = Trending::from_str(&body)?;
    Ok(videos)
}

pub fn popular(server: &str, args: Option<&str>) -> Result<Popular, Box<dyn Error>> {
    let body = reqwest::blocking::get(Popular::url(server, url_args(None, args)))?.text()?;
    let videos = Popular::from_str(&body)?;
    Ok(videos)
}

pub fn channel(server: &str, id: &str, args: Option<&str>) -> Result<Channel, Box<dyn Error>> {
    let body = reqwest::blocking::get(Channel::url(server, url_args(Some(id), args)))?.text()?;
    let channel = Channel::from_str(&body)?;
    Ok(channel)
}

pub fn channel_videos(
    server: &str,
    id: &str,
    args: Option<&str>,
) -> Result<ChannelVideos, Box<dyn Error>> {
    let body =
        reqwest::blocking::get(ChannelVideos::url(server, url_args(Some(id), args)))?.text()?;
    let videos = ChannelVideos::from_str(&body)?;
    Ok(videos)
}

pub fn channel_playlists(
    server: &str,
    id: &str,
    args: Option<&str>,
) -> Result<ChannelPlaylists, Box<dyn Error>> {
    let body =
        reqwest::blocking::get(ChannelPlaylists::url(server, url_args(Some(id), args)))?.text()?;
    let playlists = ChannelPlaylists::from_str(&body)?;
    Ok(playlists)
}

pub fn channel_comments(
    server: &str,
    id: &str,
    args: Option<&str>,
) -> Result<ChannelComments, Box<dyn Error>> {
    let body =
        reqwest::blocking::get(ChannelComments::url(server, url_args(Some(id), args)))?.text()?;
    let comments = ChannelComments::from_str(&body)?;
    Ok(comments)
}

pub fn channel_search(
    server: &str,
    id: &str,
    args: Option<&str>,
) -> Result<ChannelSearch, Box<dyn Error>> {
    let body =
        reqwest::blocking::get(ChannelSearch::url(server, url_args(Some(id), args)))?.text()?;
    let search = ChannelSearch::from_str(&body)?;

    Ok(search)
}

pub fn search(server: &str, args: Option<&str>) -> Result<Search, Box<dyn Error>> {
    let body = reqwest::blocking::get(Search::url(server, url_args(None, args)))?.text()?;
    let search = Search::from_str(&body)?;
    Ok(search)
}

pub fn playlist(server: &str, id: &str, args: Option<&str>) -> Result<Playlist, Box<dyn Error>> {
    let body = reqwest::blocking::get(Playlist::url(server, url_args(Some(id), args)))?.text()?;
    let playlist = Playlist::from_str(&body)?;
    Ok(playlist)
}

pub fn mix(server: &str, id: &str, args: Option<&str>) -> Result<Mix, Box<dyn Error>> {
    let body = reqwest::blocking::get(Mix::url(server, url_args(Some(id), args)))?.text()?;
    let mix = Mix::from_str(&body)?;
    Ok(mix)
}

#[cfg(any(feature = "sync", feature = "async"))]
use std::error::Error;

use crate::InvidiousError;
#[cfg(any(feature = "sync", feature = "async"))]
use crate::{channel::*, universal::*, video::*, PublicItems};

#[cfg(feature = "sync")]
/// Sync Invidious client
pub trait ClientSyncTrait
where
    Self: Sized,
{
    /// Creates new ClientSync from a given instance and method.
    fn new(instance: String) -> Self;

    /// Modifies the instance of the ClientSync.
    fn set_instance(&mut self, instance: String);

    /// Returns the currently in use instance.
    fn get_instance(&self) -> &str;

    /// Takes ownership of the instance and returns a new, modifed ClientSync with changed instance.
    fn instance(mut self, instance: String) -> Self {
        self.set_instance(instance);
        self
    }

    /// Sends an http get request to the url and returns result.
    fn fetch(&self, url: &str) -> Result<String, Box<dyn Error>>;

    /// `/api/v1/stats` endpoint.
    fn stats(&self, params: Option<&str>) -> Result<Stats, InvidiousError> {
        Stats::fetch_sync(self, None, params)
    }

    /// `/api/v1/videos/:ID` endpoint.
    fn video(&self, id: &str, params: Option<&str>) -> Result<Video, InvidiousError> {
        Video::fetch_sync(self, Some(id), params)
    }

    /// `/api/v1/comments/:ID` endpoint.
    fn comments(&self, id: &str, params: Option<&str>) -> Result<Comments, InvidiousError> {
        Comments::fetch_sync(self, Some(id), params)
    }

    /// `/api/v1/captions/:id` endpoint.
    fn captions(&self, id: &str, params: Option<&str>) -> Result<Captions, InvidiousError> {
        Captions::fetch_sync(self, Some(id), params)
    }

    /// `/api/v1/annotations/:id` endpoint.
    fn annotations(&self, id: &str, params: Option<&str>) -> Result<Annotations, InvidiousError> {
        Annotations::fetch_sync(self, Some(id), params)
    }

    /// `/api/v1/trending` endpoint.
    fn trending(&self, params: Option<&str>) -> Result<Trending, InvidiousError> {
        Trending::fetch_sync(self, None, params)
    }

    /// `/api/v1/popular` endpoint.
    fn popular(&self, params: Option<&str>) -> Result<Popular, InvidiousError> {
        Popular::fetch_sync(self, None, params)
    }

    /// `/api/v1/channels/:ID` endpoint.
    fn channel(&self, id: &str, params: Option<&str>) -> Result<Channel, InvidiousError> {
        Channel::fetch_sync(self, Some(id), params)
    }

    /// `/api/v1/channel/videos/:ID` endpoint.
    fn channel_videos(
        &self,
        id: &str,
        params: Option<&str>,
    ) -> Result<ChannelVideos, InvidiousError> {
        ChannelVideos::fetch_sync(self, Some(id), params)
    }

    /// `/api/v1/channel/playlists/:ID` endpoint.
    fn channel_playlists(
        &self,
        id: &str,
        params: Option<&str>,
    ) -> Result<ChannelPlaylists, InvidiousError> {
        ChannelPlaylists::fetch_sync(self, Some(id), params)
    }

    /// `/api/v1/channel/comments/:id` endpoint.
    fn channel_comments(
        &self,
        id: &str,
        params: Option<&str>,
    ) -> Result<ChannelComments, InvidiousError> {
        ChannelComments::fetch_sync(self, Some(id), params)
    }

    /// `/api/v1/channel/search/:id` endpoint.
    fn channel_search(
        &self,
        id: &str,
        params: Option<&str>,
    ) -> Result<ChannelSearch, InvidiousError> {
        ChannelSearch::fetch_sync(self, Some(id), params)
    }

    /// `/api/v1/search` endpoint.
    fn search(&self, params: Option<&str>) -> Result<Search, InvidiousError> {
        Search::fetch_sync(self, None, params)
    }

    /// `/api/v1/playlists/:ID` endpoint.
    fn playlist(&self, id: &str, params: Option<&str>) -> Result<Playlist, InvidiousError> {
        Playlist::fetch_sync(self, Some(id), params)
    }

    /// `/api/v1/mixes/:ID` endpoint.
    fn mix(&self, id: &str, params: Option<&str>) -> Result<Mix, InvidiousError> {
        Mix::fetch_sync(self, Some(id), params)
    }

    /// `/api/v1/search/suggestions` endpoint.
    fn search_suggestions(&self, params: Option<&str>) -> Result<Suggestions, InvidiousError> {
        Suggestions::fetch_sync(self, None, params)
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait]
/// Async Invidious client
///
/// Please read docs before impl this to your own struct
///
/// For the fetch function, don't actually impl that long gibberish, it wouldn't work.
/// Instead, look into the source code and use the `#[async_trait]` macro instead.
pub trait ClientAsyncTrait
where
    Self: Sync + Sized,
{
    /// Creates new ClientSync from a given instance and method.
    fn new(instance: String) -> Self;

    /// Modifies the instance of the ClientSync.
    fn set_instance(&mut self, instance: String);

    /// Returns the currently in use instance.
    fn get_instance(&self) -> &str;

    /// Takes ownership of the instance and returns a new, modifed ClientSync with changed instance.
    fn instance(mut self, instance: String) -> Self {
        self.set_instance(instance);
        self
    }

    /// Sends an http get request to the url and returns result.
    async fn fetch(&self, url: &str) -> Result<String, Box<dyn Error>>;

    /// `/api/v1/stats` endpoint.
    async fn stats(&self, params: Option<&str>) -> Result<Stats, InvidiousError> {
        Stats::fetch_async(self, None, params).await
    }

    /// `/api/v1/videos/:ID` endpoint.
    async fn video(&self, id: &str, params: Option<&str>) -> Result<Video, InvidiousError> {
        Video::fetch_async(self, Some(id), params).await
    }

    /// `/api/v1/comments/:ID` endpoint.
    async fn comments(&self, id: &str, params: Option<&str>) -> Result<Comments, InvidiousError> {
        Comments::fetch_async(self, Some(id), params).await
    }

    /// `/api/v1/comments/:ID` endpoint.
    async fn captions(&self, id: &str, params: Option<&str>) -> Result<Captions, InvidiousError> {
        Captions::fetch_async(self, Some(id), params).await
    }

    /// `/api/v1/comments/:ID` endpoint.
    async fn annotations(
        &self,
        id: &str,
        params: Option<&str>,
    ) -> Result<Annotations, InvidiousError> {
        Annotations::fetch_async(self, Some(id), params).await
    }

    /// `/api/v1/trending` endpoint.
    async fn trending(&self, params: Option<&str>) -> Result<Trending, InvidiousError> {
        Trending::fetch_async(self, None, params).await
    }

    /// `/api/v1/popular` endpoint.
    async fn popular(&self, params: Option<&str>) -> Result<Popular, InvidiousError> {
        Popular::fetch_async(self, None, params).await
    }

    /// `/api/v1/channels/:ID` endpoint.
    async fn channel(&self, id: &str, params: Option<&str>) -> Result<Channel, InvidiousError> {
        Channel::fetch_async(self, Some(id), params).await
    }

    /// `/api/v1/channel/videos/:ID` endpoint.
    async fn channel_videos(
        &self,
        id: &str,
        params: Option<&str>,
    ) -> Result<ChannelVideos, InvidiousError> {
        ChannelVideos::fetch_async(self, Some(id), params).await
    }

    /// `/api/v1/channel/playlists/:ID` endpoint.
    async fn channel_playlists(
        &self,
        id: &str,
        params: Option<&str>,
    ) -> Result<ChannelPlaylists, InvidiousError> {
        ChannelPlaylists::fetch_async(self, Some(id), params).await
    }

    /// `/api/v1/channel/playlists/:ID` endpoint.
    async fn channel_comments(
        &self,
        id: &str,
        params: Option<&str>,
    ) -> Result<ChannelComments, InvidiousError> {
        ChannelComments::fetch_async(self, Some(id), params).await
    }

    /// `/api/v1/channel/playlists/:ID` endpoint.
    async fn channel_search(
        &self,
        id: &str,
        params: Option<&str>,
    ) -> Result<ChannelSearch, InvidiousError> {
        ChannelSearch::fetch_async(self, Some(id), params).await
    }

    /// `/api/v1/search` endpoint.
    async fn search(&self, params: Option<&str>) -> Result<Search, InvidiousError> {
        Search::fetch_async(self, None, params).await
    }

    /// `/api/v1/playlists/:ID` endpoint.
    async fn playlist(&self, id: &str, params: Option<&str>) -> Result<Playlist, InvidiousError> {
        Playlist::fetch_async(self, Some(id), params).await
    }

    /// `/api/v1/mixes/:ID` endpoint.
    async fn mix(&self, id: &str, params: Option<&str>) -> Result<Mix, InvidiousError> {
        Mix::fetch_async(self, Some(id), params).await
    }

    /// `/api/v1/search/suggestions` endpoint.
    async fn search_suggestions(
        &self,
        params: Option<&str>,
    ) -> Result<Suggestions, InvidiousError> {
        Suggestions::fetch_async(self, None, params).await
    }
}

#[cfg(feature = "sync")]
/// ClientSync, but with Clone
pub trait ClientSyncClone
where
    Self: ClientSyncTrait + Clone,
{
}
#[cfg(feature = "async")]
/// ClientAsync, but with Clone
pub trait ClientAsyncClone
where
    Self: ClientAsyncTrait + Clone,
{
}

#[cfg(any(feature = "sync", feature = "async"))]
use crate::{
    structs::{
        exposed::{channel::*, universal::*, video::*},
        *,
    },
    traits::PublicItems,
    INSTANCE,
};
#[cfg(any(feature = "sync", feature = "async"))]
use std::error::Error;

/// A blocking client struct, containing all info needed to perform a fetch.
#[cfg(feature = "sync")]
#[derive(Clone)]
pub struct ClientSync {
    /// Method of fetching, all methods in ClientSync are blocking methods.
    pub method: MethodSync,
    /// A valid Invidious instance url.
    pub instance: String,
}

#[cfg(feature = "sync")]
impl Default for ClientSync {
    fn default() -> Self {
        Self {
            method: MethodSync::default(),
            instance: INSTANCE.to_string(),
        }
    }
}

#[cfg(feature = "sync")]
impl ClientSync {
    /// Creates new ClientSync from a given instance and method.
    pub fn new(instance: String, method: MethodSync) -> Self {
        Self { method, instance }
    }

    /// Modifies the instance of the ClientSync.
    pub fn set_instance(&mut self, instance: String) {
        self.instance = instance;
    }

    /// Takes ownership of the instance and returns a new, modifed ClientSync with changed instance.
    pub fn instance(mut self, instance: String) -> Self {
        self.set_instance(instance);
        self
    }

    /// Modifies the method of the ClientSync.
    pub fn set_method(&mut self, method: MethodSync) {
        self.method = method;
    }

    /// Takes ownership of the method and returns a new, modifed ClientSync with changed method.
    pub fn method(mut self, method: MethodSync) -> Self {
        self.set_method(method);
        self
    }
}

#[cfg(feature = "sync")]
impl ClientSync {
    /// `/api/v1/stats` endpoint.
    pub fn stats(&self, params: Option<&str>) -> Result<Stats, Box<dyn Error>> {
        Stats::fetch_sync(self, None, params)
    }

    /// `/api/v1/videos/:ID` endpoint.
    pub fn video(&self, id: &str, params: Option<&str>) -> Result<Video, Box<dyn Error>> {
        Video::fetch_sync(self, Some(id), params)
    }

    /// `/api/v1/comments/:ID` endpoint.
    pub fn comments(&self, id: &str, params: Option<&str>) -> Result<Comments, Box<dyn Error>> {
        Comments::fetch_sync(self, Some(id), params)
    }

    /// `/api/v1/captions/:id` endpoint.
    pub fn captions(&self, id: &str, params: Option<&str>) -> Result<Captions, Box<dyn Error>> {
        Captions::fetch_sync(self, Some(id), params)
    }

    /// `/api/v1/annotations/:id` endpoint.
    pub fn annotations(
        &self,
        id: &str,
        params: Option<&str>,
    ) -> Result<Annotations, Box<dyn Error>> {
        Annotations::fetch_sync(self, Some(id), params)
    }

    /// `/api/v1/trending` endpoint.
    pub fn trending(&self, params: Option<&str>) -> Result<Trending, Box<dyn Error>> {
        Trending::fetch_sync(self, None, params)
    }

    /// `/api/v1/popular` endpoint.
    pub fn popular(&self, params: Option<&str>) -> Result<Popular, Box<dyn Error>> {
        Popular::fetch_sync(self, None, params)
    }

    /// `/api/v1/channel/:ID` endpoint.
    pub fn channel(&self, id: &str, params: Option<&str>) -> Result<Channel, Box<dyn Error>> {
        Channel::fetch_sync(self, Some(id), params)
    }

    /// `/api/v1/channel/videos/:ID` endpoint.
    pub fn channel_videos(
        &self,
        id: &str,
        params: Option<&str>,
    ) -> Result<ChannelVideos, Box<dyn Error>> {
        ChannelVideos::fetch_sync(self, Some(id), params)
    }

    /// `/api/v1/channel/playlists/:ID` endpoint.
    pub fn channel_playlists(
        &self,
        id: &str,
        params: Option<&str>,
    ) -> Result<ChannelPlaylists, Box<dyn Error>> {
        ChannelPlaylists::fetch_sync(self, Some(id), params)
    }

    /// `/api/v1/channel/comments/:id` endpoint.
    pub fn channel_comments(
        &self,
        id: &str,
        params: Option<&str>,
    ) -> Result<ChannelComments, Box<dyn Error>> {
        ChannelComments::fetch_sync(self, Some(id), params)
    }

    /// `/api/v1/channel/search/:id` endpoint.
    pub fn channel_search(
        &self,
        id: &str,
        params: Option<&str>,
    ) -> Result<ChannelSearch, Box<dyn Error>> {
        ChannelSearch::fetch_sync(self, Some(id), params)
    }

    /// `/api/v1/search` endpoint.
    pub fn search(&self, params: Option<&str>) -> Result<Search, Box<dyn Error>> {
        Search::fetch_sync(self, None, params)
    }

    /// `/api/v1/playlists/:ID` endpoint.
    pub fn playlist(&self, id: &str, params: Option<&str>) -> Result<Playlist, Box<dyn Error>> {
        Playlist::fetch_sync(self, Some(id), params)
    }

    /// `/api/v1/mixes/:ID` endpoint.
    pub fn mix(&self, id: &str, params: Option<&str>) -> Result<Mix, Box<dyn Error>> {
        Mix::fetch_sync(self, Some(id), params)
    }
}

/// An async client, containing all info needed to perform a fetch.
#[cfg(feature = "async")]
pub struct ClientAsync {
    /// Method of fetching, all methods in ClientAsync are async methods.
    pub method: MethodAsync,
    /// A valid Invidious instance url.
    pub instance: String,
}

#[cfg(feature = "async")]
impl Default for ClientAsync {
    fn default() -> Self {
        Self {
            method: MethodAsync::default(),
            instance: INSTANCE.to_string(),
        }
    }
}

#[cfg(feature = "async")]
impl ClientAsync {
    /// Creates new ClientAsync from a given instance and method.
    pub fn new(instance: String, method: MethodAsync) -> Self {
        Self { method, instance }
    }

    /// Modifies the instance of the ClientAsync.
    pub fn set_instance(&mut self, instance: String) {
        self.instance = instance;
    }

    /// Takes ownership of the instance and returns a new, modifed ClientAsync with changed instance.
    pub fn instance(mut self, instance: String) -> Self {
        self.set_instance(instance);
        self
    }

    /// Modifies the method of the ClientAsync.
    pub fn set_method(&mut self, method: MethodAsync) {
        self.method = method;
    }

    /// Takes ownership of the method and returns a new, modifed ClientAsync with changed method.
    pub fn method(mut self, method: MethodAsync) -> Self {
        self.set_method(method);
        self
    }
}

#[cfg(feature = "async")]
impl ClientAsync {
    /// `/api/v1/stats` endpoint.
    pub async fn stats(&self, params: Option<&str>) -> Result<Stats, Box<dyn Error>> {
        Stats::fetch_async(self, None, params).await
    }

    /// `/api/v1/videos/:ID` endpoint.
    pub async fn video(&self, id: &str, params: Option<&str>) -> Result<Video, Box<dyn Error>> {
        Video::fetch_async(self, Some(id), params).await
    }

    /// `/api/v1/comments/:ID` endpoint.
    pub async fn comments(
        &self,
        id: &str,
        params: Option<&str>,
    ) -> Result<Comments, Box<dyn Error>> {
        Comments::fetch_async(self, Some(id), params).await
    }

    /// `/api/v1/comments/:ID` endpoint.
    pub async fn captions(
        &self,
        id: &str,
        params: Option<&str>,
    ) -> Result<Captions, Box<dyn Error>> {
        Captions::fetch_async(self, Some(id), params).await
    }

    /// `/api/v1/comments/:ID` endpoint.
    pub async fn annotations(
        &self,
        id: &str,
        params: Option<&str>,
    ) -> Result<Annotations, Box<dyn Error>> {
        Annotations::fetch_async(self, Some(id), params).await
    }

    /// `/api/v1/trending` endpoint.
    pub async fn trending(&self, params: Option<&str>) -> Result<Trending, Box<dyn Error>> {
        Trending::fetch_async(self, None, params).await
    }

    /// `/api/v1/popular` endpoint.
    pub async fn popular(&self, params: Option<&str>) -> Result<Popular, Box<dyn Error>> {
        Popular::fetch_async(self, None, params).await
    }

    /// `/api/v1/channel/:ID` endpoint.
    pub async fn channel(&self, id: &str, params: Option<&str>) -> Result<Channel, Box<dyn Error>> {
        Channel::fetch_async(self, Some(id), params).await
    }

    /// `/api/v1/channel/videos/:ID` endpoint.
    pub async fn channel_videos(
        &self,
        id: &str,
        params: Option<&str>,
    ) -> Result<ChannelVideos, Box<dyn Error>> {
        ChannelVideos::fetch_async(self, Some(id), params).await
    }

    /// `/api/v1/channel/playlists/:ID` endpoint.
    pub async fn channel_playlists(
        &self,
        id: &str,
        params: Option<&str>,
    ) -> Result<ChannelPlaylists, Box<dyn Error>> {
        ChannelPlaylists::fetch_async(self, Some(id), params).await
    }

    /// `/api/v1/channel/playlists/:ID` endpoint.
    pub async fn channel_comments(
        &self,
        id: &str,
        params: Option<&str>,
    ) -> Result<ChannelComments, Box<dyn Error>> {
        ChannelComments::fetch_async(self, Some(id), params).await
    }

    /// `/api/v1/channel/playlists/:ID` endpoint.
    pub async fn channel_search(
        &self,
        id: &str,
        params: Option<&str>,
    ) -> Result<ChannelSearch, Box<dyn Error>> {
        ChannelSearch::fetch_async(self, Some(id), params).await
    }

    /// `/api/v1/search` endpoint.
    pub async fn search(&self, params: Option<&str>) -> Result<Search, Box<dyn Error>> {
        Search::fetch_async(self, None, params).await
    }

    /// `/api/v1/playlists/:ID` endpoint.
    pub async fn playlist(
        &self,
        id: &str,
        params: Option<&str>,
    ) -> Result<Playlist, Box<dyn Error>> {
        Playlist::fetch_async(self, Some(id), params).await
    }

    /// `/api/v1/mixes/:ID` endpoint.
    pub async fn mix(&self, id: &str, params: Option<&str>) -> Result<Mix, Box<dyn Error>> {
        Mix::fetch_async(self, Some(id), params).await
    }
}

use crate::{
    structs::{channel::*, universal::*, video::*},
    traits::*,
};
use std::error::Error;

/// Main struct of the library with all the functions to interact with the Invidious API (Async).
///
/// # Examples
/// ```rust
/// use invidious::asynchronous::Client;
/// use std::error::Error;
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///    // Crate a new client with a valid address (without the trailing slash)
///    let client = Client::new(String::from("https://vid.puffyan.us"));
///
///    // Get information about a video
///    let video = client.video("dQw4w9WgXcQ", None).await?; // dQw4w9WgXcQ is the id of the video, None is the extra arguments for the request
///    println!("{}", video.title); // "Rick Astley - Never Gonna Give You Up (Official Music Video)"
///    
///    Ok(())
/// }
/// ```
/// 
/// # Additional arguments
/// Additional arguments can be passed in to the functions, such as the language, the page, the sorting, etc. Arguments are optional and differs in the different functions.
/// 
/// The arguments passed in as `Option<&str>` which can be `None`. To pass in an argument, do `Some("argument_name=argument_value")`, to pass in two or more arguments, do `Some("argument_name1=argument_value1&argument_name2=argument_value2")`.
/// 
/// # Errors
/// There are two main sources of errors:
/// 
/// * Reqest: Failed to connect to the Invidious instance.
/// * Serde_json: Failed to parse the response from the Invidious instance. This is most likely caused by an invalid response from the Invidious instance, such as an error message from the server (which is not in an expected JSON structure), or the Invidious API is returning a JSON with a different structure than the expected JSON in their documentation (most of their responses are, but I made it work by changing the expected structure of the response, also this is not the crate's fault it Invidious' problem).

#[derive(Debug, Clone)]
pub struct Client {
    server: String,
}

impl Client {

    /// Creates a new async client with the given server address.
    ///
    /// # Examples
    /// ```rust
    /// # use invidious::asynchronous::Client;
    /// #
    /// # fn main() {
    /// let client = Client::new(String::from("https://vid.puffyan.us"));
    /// # }
    /// ```
    pub fn new(server_addr: String) -> Client {
        Client { server: server_addr }
    }

    /// Changes the server address of the client.
    ///
    /// # Examples
    /// ```rust
    /// # use invidious::asynchronous::Client;
    /// #
    /// # fn main() {
    /// let mut client = Client::new(String::from("https://vid.puffyan.us"));
    /// client.change_srv(String::from("https://tube.cthd.icu"));
    /// # }
    /// ```
    pub fn change_srv(&mut self, new_server_addr: String) {
        self.server = new_server_addr;
    }

    /// Get information about the Invidious instance.
    ///
    /// # Examples
    /// ```rust
    /// # use invidious::asynchronous::Client;
    /// # use std::error::Error;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn Error>> {
    /// # let client = Client::new(String::from("https://vid.puffyan.us"));
    /// let stats = client.stats(None).await?;
    /// # Ok(())
    /// # }
    /// ```    
    /// 
    /// # Additional arguments
    /// (No additional arguments)
    pub async fn stats(&self, args: Option<&str>) -> Result<Stats, Box<dyn Error>> {
        let body = reqwest::get(Stats::url(&self.server, url_args(None, args)))
            .await?
            .text()
            .await?;
        let stats = Stats::from_str(&body)?;
        Ok(stats)
    }
    
    /// Get information about a video.
    ///
    /// # Examples
    /// ```rust
    /// # use invidious::asynchronous::Client;
    /// # use std::error::Error;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn Error>> {
    /// # let client = Client::new(String::from("https://vid.puffyan.us"));
    /// let video = client.video("dQw4w9WgXcQ", None).await?;
    /// # Ok(())
    /// # }
    /// ```
    /// 
    /// # Additional arguments
    /// * `region`: ISO 3166 country code (default: `US`)
    pub async fn video(&self, video_id: &str, args: Option<&str>) -> Result<Video, Box<dyn Error>> {
        let body = reqwest::get(Video::url(&self.server, url_args(Some(video_id), args)))
            .await?
            .text()
            .await?;
        let video = Video::from_str(&body)?;
        Ok(video)
    }

    /// Get comments of a video.
    /// 
    /// # Examples
    /// ```rust
    /// # use invidious::asynchronous::Client;
    /// # use std::error::Error;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn Error>> {
    /// # let client = Client::new(String::from("https://vid.puffyan.us"));
    /// let comments = client.comments("MSfD-QApDyU", None).await?;
    /// # Ok(())
    /// # }
    /// ```
    /// 
    /// # Additional arguments
    /// * `sort_by`: `top`, `new` (default: `top`)
    /// * `source`: `youtube`, `reddit` (default: `youtube`)
    pub async fn comments(&self, video_id: &str, args: Option<&str>) -> Result<Comments, Box<dyn Error>> {
        let body = reqwest::get(Comments::url(&self.server, url_args(Some(video_id), args)))
            .await?
            .text()
            .await?;
        let comments = Comments::from_str(&body)?;
        Ok(comments)
    }

    /// Get captions of a video.
    /// 
    /// # Examples
    /// ```rust
    /// # use invidious::asynchronous::Client;
    /// # use std::error::Error;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn Error>> {
    /// # let client = Client::new(String::from("https://vid.puffyan.us"));
    /// let captions = client.captions("MSfD-QApDyU", None).await?;
    /// # Ok(())
    /// # }
    /// ```
    /// 
    /// # Additional arguments
    /// 
    /// A request with `label` will return the selected captions in WebVTT format. Captions can also be selected with an ISO `lang`, e.g. `&lang=en`, `tlang` will auto-translate from English into the requested language (if English captions are available).
    /// 
    /// * `label`: String
    /// * `lang`:  String
    /// * `tlang`: String
    /// * `region`: ISO 3166 country code (default: `US`)
    pub async fn captions(&self, video_id: &str, args: Option<&str>) -> Result<Captions, Box<dyn Error>> {
        let body = reqwest::get(Captions::url(&self.server, url_args(Some(video_id), args)))
            .await?
            .text()
            .await?;
        let captions = Captions::from_str(&body)?;
        Ok(captions)
    }

    /// Get videos on the trending page (Same as on YouTube).
    /// 
    /// # Examples
    /// ```rust
    /// # use invidious::asynchronous::Client;
    /// # use std::error::Error;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn Error>> {
    /// # let client = Client::new(String::from("https://vid.puffyan.us"));
    /// let videos = client.trending(None).await?;
    /// # Ok(())
    /// # }
    /// ```
    /// 
    /// # Additional arguments
    /// 
    /// * `type`: `music`, `gaming`, `news`, `movies`
    /// * `region`: ISO 3166 country code (default: `US`)
    pub async fn trending(&self, args: Option<&str>) -> Result<Trending, Box<dyn Error>> {
        let body = reqwest::get(Trending::url(&self.server, url_args(None, args)))
            .await?
            .text()
            .await?;
        let videos = Trending::from_str(&body)?;
        Ok(videos)
    }

    /// Get videos on the popular page (Same as on YouTube).
    /// 
    /// # Examples
    /// ```rust
    /// # use invidious::asynchronous::Client;
    /// # use std::error::Error;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn Error>> {
    /// # let client = Client::new(String::from("https://vid.puffyan.us"));
    /// let videos = client.popular(None).await?;
    /// # Ok(())
    /// # }
    /// ```
    /// 
    /// # Additional arguments
    /// (No additional arguments)
    pub async fn popular(&self, args: Option<&str>) -> Result<Popular, Box<dyn Error>> {
        let body = reqwest::get(Popular::url(&self.server, url_args(None, args)))
            .await?
            .text()
            .await?;
        let videos = Popular::from_str(&body)?;
        Ok(videos)
    }

    /// Get channel information
    /// 
    /// ~~Note that a channel's username (if it doesn't include spaces) is also valid in place of ucid, e.g. /api/v1/channels/BlenderFoundation.~~
    /// 
    /// The official API documentation is wrong, a channel's username is actually an **invalid** ucid.
    /// 
    /// # Examples
    /// ```rust
    /// # use invidious::asynchronous::Client;
    /// # use std::error::Error;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn Error>> {
    /// # let client = Client::new(String::from("https://vid.puffyan.us"));
    /// let channel = client.channel("UC-9-kyTW8ZkZNDHQJ6FgpwQ", None).await?;
    /// # Ok(())
    /// # }
    /// ```
    /// 
    /// # Additional arguments
    /// 
    /// * `sort_by`: `newest`, `oldest`, `popular` (default: `newest`)
    pub async fn channel(&self, id: &str, args: Option<&str>) -> Result<Channel, Box<dyn Error>> {
        let body = reqwest::get(Channel::url(&self.server, url_args(Some(id), args)))
            .await?
            .text()
            .await?;
        let channel = Channel::from_str(&body)?;
        Ok(channel)
    }

    /// Get videos of a channel.
    /// 
    /// # Examples
    /// ```rust
    /// # use invidious::asynchronous::Client;
    /// # use std::error::Error;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn Error>> {
    /// # let client = Client::new(String::from("https://vid.puffyan.us"));
    /// let videos = client.channel_videos("UC-9-kyTW8ZkZNDHQJ6FgpwQ", None).await?;
    /// # Ok(())
    /// # }
    /// ```
    /// 
    /// # Additional arguments
    /// * `page`: Int32
    /// * `sort_by`: `newest`, `oldest`, `popular` (default: `newest`)
    pub async fn channel_videos(
        &self,
        id: &str,
        args: Option<&str>,
    ) -> Result<ChannelVideos, Box<dyn Error>> {
        let body = reqwest::get(ChannelVideos::url(&self.server, url_args(Some(id), args)))
            .await?
            .text()
            .await?;
        let videos = ChannelVideos::from_str(&body)?;
        Ok(videos)
    }

    /// Get playlists of a channel.
    /// 
    /// # Examples
    /// ```rust
    /// # use invidious::asynchronous::Client;
    /// # use std::error::Error;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn Error>> {
    /// # let client = Client::new(String::from("https://vid.puffyan.us"));
    /// let playlists = client.channel_playlists("UC-9-kyTW8ZkZNDHQJ6FgpwQ", None).await?;
    /// # Ok(())
    /// # }
    /// ```
    /// 
    /// # Additional arguments
    /// * `continuation`: String
    /// * `sort_by`: `oldest`, `newest`, `last`
    pub async fn channel_playlists(
        &self,
        id: &str,
        args: Option<&str>,
    ) -> Result<ChannelPlaylists, Box<dyn Error>> {
        let body = reqwest::get(ChannelPlaylists::url(
            &self.server,
            url_args(Some(id), args),
        ))
        .await?
        .text()
        .await?;
        let playlists = ChannelPlaylists::from_str(&body)?;
        Ok(playlists)
    }

    /// Get comments of a channel.
    /// 
    /// # Examples
    /// ```rust
    /// # use invidious::asynchronous::Client;
    /// # use std::error::Error;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn Error>> {
    /// # let client = Client::new(String::from("https://vid.puffyan.us"));
    /// let comments = client.channel_comments("UC-9-kyTW8ZkZNDHQJ6FgpwQ", None).await?;
    /// # Ok(())
    /// # }
    /// ```
    /// 
    /// # Additional arguments
    /// * `continuation`: String
    pub async fn channel_comments(
        &self,
        id: &str,
        args: Option<&str>,
    ) -> Result<ChannelComments, Box<dyn Error>> {
        let body = reqwest::get(ChannelComments::url(&self.server, url_args(Some(id), args)))
            .await?
            .text()
            .await?;
        let comments = ChannelComments::from_str(&body)?;
        Ok(comments)
    }

    /// Search anything from the channel.
    /// 
    /// # Examples
    /// ```rust
    /// # use invidious::asynchronous::Client;
    /// # use std::error::Error;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn Error>> {
    /// # let client = Client::new(String::from("https://vid.puffyan.us"));
    /// let search = client.channel_search("UCkl7SSSoiMuHuhskQpycVuA", Some("q=hello")).await?;
    /// # Ok(())
    /// # }
    /// ```
    /// 
    /// # Additional arguments
    /// * `q`: String (Required)
    /// * `page`: Int32
    pub async fn channel_search(&self, id: &str, args: Option<&str>) -> Result<ChannelSearch, Box<dyn Error>> {
        let body = reqwest::get(ChannelSearch::url(&self.server, url_args(Some(id), args)))
            .await?
            .text()
            .await?;
        let search = ChannelSearch::from_str(&body)?;

        Ok(search)
    }

    /// Search anything in YouTube
    /// 
    /// # Examples
    /// ```rust
    /// # use invidious::asynchronous::Client;
    /// # use std::error::Error;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn Error>> {
    /// # let client = Client::new(String::from("https://vid.puffyan.us"));
    /// let search = client.search(Some("q=hello")).await?;
    /// # Ok(())
    /// # }
    /// ```
    /// 
    /// # Additional arguments
    /// * `q`: String
    /// * `page`: Int32
    /// * `sort_by`: `relevance`, `rating`, `upload_date`, `view_count`
    /// * `date`: `hour`, `today`, `week`, `month`, `year`
    /// * `duration`: `short`, `long`
    /// * `type`: `video`, `playlist`, `channel`, `all`, (default: `video`)
    /// * `features`: `hd`, `subtitles`, `creative_commons`, `3d`, `live`, `purchased`, `4k`, `360`, `location`, `hdr` (comma separated: e.g. `&features=hd,subtitles,3d,live`)
    /// * `region`: ISO 3166 country code (default: `US`)

    pub async fn search(&self, args: Option<&str>) -> Result<Search, Box<dyn Error>> {
        let body = reqwest::get(Search::url(&self.server, url_args(None, args)))
            .await?
            .text()
            .await?;
        let search = Search::from_str(&body)?;
        Ok(search)
    }

    /// Get information of a playlist.
    /// 
    /// # Examples
    /// ```rust
    /// # use invidious::asynchronous::Client;
    /// # use std::error::Error;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn Error>> {
    /// # let client = Client::new(String::from("https://vid.puffyan.us"));
    /// let playlist = client.playlist("PLdgHTasZAjYZlCXN9rTcX9LFOQ-RIrzCs", None).await?;
    /// # Ok(())
    /// # }
    /// ```
    /// 
    /// # Additional arguments
    /// * `page`: Int32
    pub async fn playlist(&self, id: &str, args: Option<&str>) -> Result<Playlist, Box<dyn Error>> {
        let body = reqwest::get(Playlist::url(&self.server, url_args(Some(id), args)))
            .await?
            .text()
            .await?;
        let playlist = Playlist::from_str(&body)?;
        Ok(playlist)
    }

    /// Actually I had no idea what this does, but since it is on the Invidious API I decided to include it.
    /// 
    /// No examples will be provided because what is a YouTube mix? I'm so confused.
    pub async fn mix(&self, id: &str, args: Option<&str>) -> Result<Mix, Box<dyn Error>> {
        let body = reqwest::get(Mix::url(&self.server, url_args(Some(id), args)))
            .await?
            .text()
            .await?;
        let mix = Mix::from_str(&body)?;
        Ok(mix)
    }
}

fn url_args(id: Option<&str>, args: Option<&str>) -> String {
    match args {
        Some(args) => format!("{}?{}", id.unwrap_or(""), args),
        None => id.unwrap_or("").to_string(),
    }
}

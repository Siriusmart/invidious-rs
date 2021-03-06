//! # invidious
//! Rust bindings for the invidious API.
//! Get information about videos, channels, and playlists from YouTube without using the YouTube official API.
//! 
//! # Examples
//!
//! ## Blocking API
//! ```rust
//! use invidious::blocking::Client;
//! use std::error::Error;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!    let client = Client::new(String::from("https://vid.puffyan.us"));
//!    let search_results = client.search(Some("q=rust programming"))?.items;
//!    let video = client.video("5C_HPTJg5ek", None)?;
//!
//!   Ok(())
//! }
//! ```
//!
//! ## Async API
//! ```rust
//! use invidious::asynchronous::Client;
//! use std::error::Error;
//!
//! #[tokio::main]
//! async fn main() {
//!   let client = Client::new(String::from("https://vid.puffyan.us"));
//!   let channel_videos = client.channel_videos("UCAkuTH35kk3W1EL9vq6dj6A", Some("sort_by=popular&page=2"))
//!     .await
//!     .unwrap();
//!   
//!   let playlist = client.playlist("PLFgquLnL59alCl_2TQvOiD5Vgm1hCaGSI", None)
//!     .await
//!     .unwrap(); // Returns Playlist
//! }
//! ```
//! 
//! # General Usage
//! 
//! `client.function_name(id: &str, args: Option<&str>) -> Result<T, Box<dyn Error>>`
//! 
//! * `id` is the id of the video, channel, or playlist, and is only used when applicable.
//! * `args` is an optional string of additional arguments to be passed to the API. For example, `sort_by=popular&page=2` (Arguments are separated by `&`)
//! 
//! # How this works
//! Uses the Invidious api to get information about videos, channels, and playlists. The Invidious API is a REST API, so the invidious crate uses the reqwest crate to make requests to the Invidious API, and then uses the serde_json crate to parse the JSON returned by the Invidious API.
//! 
//! Official documentation of the Invidious API can be found here: <https://docs.invidious.io/api>
//! 
//! //! # Patches
//! 
//! 0.2.1: Make `init` and `index` in AdaptiveFormat optional because some videos does not have them.
//! 
//! # License
//! 
//! GNU General Public License v3.0


pub mod asynchronous;
pub mod blocking;
pub mod errors;
pub mod structs;
pub mod tests;
pub mod traits;

pub mod private_functions;
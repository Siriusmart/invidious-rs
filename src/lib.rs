//! Rust bindings for the [Invidious API](https://docs.invidious.io/api).
//!
//! > "May Invidious live and prosper, with, or without us."
//! >
//! > ~ The Invidious team on [YouTube trying to take them down](https://github.com/iv-org/invidious/issues/3872).
//!
//! Go show them some support by [donating](https://invidious.io/donate/) and starring their
//! [GitHub repos](https://github.com/iv-org)!
//!
//! ## Quickstart
//!
//! Get started by creating a client with `ClientSync::default()` and use the functions from there.
//!
//! ### Blocking API
//!
//! ```
//! use invidious::*;
//!
//! fn main() {
//!     let client = ClientSync::default();
//!     let video = client.video("fBj3nEdCjkY", None).unwrap();
//!     let seach_results = client.search(Some("q=testing")).unwrap();
//! }
//! ```
//!
//! ### Async API
//!
//! Enable feature `reqwest_async`.
//!
//! ```toml
//! invidious = { version = "0.6", no-default-features = true, features = ["reqwest_async"]}
//! ```
//!
//! ```
//! # use invidious::*;
//! #[tokio::main]
//! async fn main() {
//!     let client = ClientAsync::default();
//!     let vidio = client.video("fBj3nEdCjkY", None).await.unwrap();
//!     let seach_results = client.search(Some("q=testing")).await.unwrap();
//! }
//! ```
//!
//! Read more about [`ClientSync`](./struct.ClientSync.html) and [`ClientAsync`](./struct.ClientAsync.html) to see all avaliable functions.
//!
//! ## Methods
//!
//! Control how the client is making the web requests.
//!
//! ### Changing methods
//!
//! For example, to use `isahc` instead of `reqwest` for sending requests in `ClientAsync`, first
//! enable the `isahc_async` feature and optionally disable the `reqwest_async` feature (if
//! enabled).
//!
//! ```toml
//! invidious = { version = "6.0", no-default-features = true, features = ["isahc_async"]}
//! ```
//!
//! ```
//! # use invidious::*;
//! # #[tokio::main]
//! # async fn main() {
//! let video = ClientAsync::default()
//!     .method(MethodAsync::ISAHC)
//!     .video("fBj3nEdCjkY", None).await.unwrap();
//! # }
//! ```
//!
//! ### Custom methods
//!
//! If the existing methods are not suitable for use (takes too long to compile, resource
//! inefficient, does not support async runtime), you can use your own fetch function instead.
//!
//! ```toml
//! invidious = { version = "6.0", no-default-features = true, features = ["async"]}
//! ```
//!
//! See no fetch features are enabled, just `async` which enables `ClientAsync` and related code.
//!
//! ```
//! # use invidious::*;
//! #[tokio::main]
//! async fn main() {
//!     let video = ClientAsync::default()
//!         .custom_method(Box::new(custom_fetch))
//!         .video("fBj3nEdCjkY", None).await.unwrap();
//! }
//!
//! // MethodReturn is just short for `Result<String, Box<dyn Error>>`
//! async fn custom_fetch(url: String) -> MethodReturn {
//!     Ok(reqwest::get(url).await?.text().await?)
//! }
//! ```
//!
//! ## Features
//!
//! This crate utilises features for specifying which crates to use for fetching the http(s) responses. Only crates that are needed are included. Different features result in various compile times and performances. The compile times of each feature can be found in [`MethodSync`](./enum.MethodSync.html) and [`MethodAsync`](./enum.MethodAsync.html).
//!
//! All avaliable features are listed below.
//!
//! |Feature|Crate used|
//! |---|---|
//! |`httpreq_sync` (enabled by default)|[http_req](https://crates.io/crates/http_req)|
//! |`ureq_sync`|[ureq](https://crates.io/crates/ureq)|
//! |`minreq_sync`|[minreq](https://crates.io/crates/minreq) with https feature|
//! |`minreq_http_sync`|[minreq](https://crates.io/crates/minreq)|
//! |`reqwest_sync`|[reqwest](https://crates.io/crates/reqwest) with blocking feature|
//! |`reqwest_async`|[reqwest](https://crates.io/crates/reqwest)|
//! |`isahc_sync`|[isahc](https://crates.io/crates/isahc)|
//! |`isahc_async`|[isahc](https://crates.io/crates/isahc)|
//!
//! ## General usage
//!
//! Most functions look something like:
//!
//! ```rs
//! client.function_name(id: &str, params: Option<&str>) // when id is needed.
//! client.function_name(params: Option<&str>) // when id is not needed, for example search.
//! ```
//!
//! ### Params
//!
//! Params allows user to include URL params as specified in the [Invidious API docs](https://docs.invidious.io/api).
//!
//! The beginning question mark `?` is omitted.
//!
//! ## How this works
//!
//! [Invidious](https://invidious.io) is an alternative frontend for YouTube, and also offering an API.
//!
//! This crate includes structs for each of the API endpoints, and allowing users to include any extra parameters they want. Then uses the [serde](https://crates.io/crates/serde) crate to serialize and deserialize json responses from the [Invidious API](https://docs.invidious.io/api).
//!
//! ## Contributing
//!
//! Contributions are welcome! Make a pull request at [GitHub](https://github.com/siriusmart/invidious-rs) if you do.
//!
//! - Make changes to outdated bindings structs.
//! - Add new fetch methods with either faster compile time or runtime.
//! - Improve documentation.

#![allow(clippy::module_inception)]
#![allow(clippy::needless_doctest_main)]

mod errors;
pub mod functions;
mod structs;
mod tests;
mod traits;

pub use errors::InvidiousError;
pub use structs::*;
pub use traits::PublicItems;

/// Default instance used in `ClientSync/ClientAsync::default()`.
pub const INSTANCE: &str = "https://vid.puffyan.us";

#[cfg(feature = "httpreq_sync")]
pub use http_req;
#[cfg(any(feature = "isahc_sync", feature = "isahc_async"))]
pub use isahc;
#[cfg(any(feature = "minreq_sync", feature = "minreq_http_sync"))]
pub use minreq;
#[cfg(any(feature = "reqwest_sync", feature = "reqwest_async"))]
pub use reqwest;
#[cfg(feature = "ureq_sync")]
pub use ureq;

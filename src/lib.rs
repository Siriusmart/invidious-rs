//! Rust bindings for the [Invidious API](https://docs.invidious.io/api).
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
//! invidious = { version = "0.7", no-default-features = true, features = ["isahc_async"]}
//! ```
//!
//! ```
//! # use invidious::*;
//! # #[tokio::main]
//! # async fn main() {
//! let video = ClientAsync::default()
//!     .method(MethodAsync::Isahc)
//!     .video("fBj3nEdCjkY", None).await.unwrap();
//! # }
//! ```
//!
//! If none of the fetch methods matches your needs, consider implmenting your own client struct.
//! ([Sync](./trait.ClientSyncTrait.html) and [async](./trait.ClientAsyncTrait.html))
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
//! On a related note, you may use [vidiup.siri.sh](https://github.com/siriusmart/vidiup) to look
//! for instances with specific features enabled.
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
mod instance;

pub use errors::InvidiousError;
pub use structs::*;
pub use traits::*;
pub use instance::INSTANCE;

/// Some instances required a user agent to be set, but it is not implmented yet
pub const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[cfg(feature = "async")]
pub use async_trait;
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

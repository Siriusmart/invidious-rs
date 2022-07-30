/// A client struct only holds the server url, and passes the url to the blocking functions when needed.
mod client;
pub use client::*;

/// All the actual functions in this crate, for how to use please read the documentation for the client struct
pub mod functions;

//! Misc functions used in the crate.
mod url_params;
pub use url_params::*;
#[cfg(feature = "httpreq_sync")]
mod httpreq_get;
#[cfg(feature = "httpreq_sync")]
pub use httpreq_get::*;

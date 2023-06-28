#![allow(unreachable_code)]
#![allow(unreachable_patterns)]

use std::{future::Future, pin::Pin};

/// Type of a custom sync fetch function
pub type MethodSyncCustom = Box<dyn Fn(&str) -> MethodReturn>;
/// Type of a custom async fetch function
pub type AsyncMethodCustom =
    Box<dyn Fn(String) -> Pin<Box<dyn Future<Output = MethodReturn> + Send>> + Send + Sync>;
/// Return type of fetch functions
pub type MethodReturn = Result<String, Box<dyn std::error::Error>>;

/// Request methods for `ClientSync`.
///
/// ## Binary build times
///
/// Build times are tested on my local Arch Linux machine, with `--release` flag..
///
/// |Feature|Time(s)|Crate|
/// |---|---|---|
/// |None|6.9|-|
/// |httpreq_sync (default)|11.4|[http_req](https://crates.io/crates/http_req)|
/// |minreq_sync|11.6|[minreq](https://crates.io/crates/minreq)|
/// |ureq_sync|13.1|[ureq](https://crates.io/crates/ureq)|
/// |reqwest_sync|18.0|[reqwest](https://crates.io/crates/reqwest)|
/// |isahc_sync|22.6|[isahc](https://crates.io/crates/isahc)|
/// |minreq_http_sync|10.1|[minreq](https://crates.io/crates/minreq)|
#[cfg(feature = "sync")]
#[derive(Clone, Copy)]
pub enum MethodSync {
    /// Default - Quickest compile time, but part of the fetching function is written by me, so it might be erroneous.
    #[cfg(feature = "httpreq_sync")]
    HttpReq,
    /// Second quickest in compile time, it comes with the fetching function, so it's less likely to fail.
    #[cfg(feature = "minreq_sync")]
    MinReq,
    /// It's just another very lightweight http client I've found.
    #[cfg(feature = "ureq_sync")]
    Ureq,
    /// Everyone knows about reqwest.
    #[cfg(feature = "reqwest_sync")]
    Reqwest,
    /// Just here because it also offers an async API.
    #[cfg(feature = "isahc_sync")]
    Isahc,
    /// Unsafe - http only, use at your own risk.
    #[cfg(feature = "minreq_http_sync")]
    MinReqHttp,
}

#[cfg(feature = "sync")]
impl Default for MethodSync {
    /// Returns the first enabled method. Panics if no methods are enabled in features.
    ///
    /// The ordering is as follows:
    /// 1. httpreq_sync
    /// 2. minreq_sync
    /// 3. ureq_sync
    /// 4. reqwest_sync
    /// 5. isahc_sync
    /// 6. minreq_http_sync
    fn default() -> Self {
        #[cfg(feature = "httpreq_sync")]
        return MethodSync::HttpReq;
        #[cfg(feature = "ureq_sync")]
        return MethodSync::Ureq;
        #[cfg(feature = "minreq_sync")]
        return MethodSync::MinReq;
        #[cfg(feature = "reqwest_sync")]
        return MethodSync::Reqwest;
        #[cfg(feature = "isahc_sync")]
        return MethodSync::Isahc;
        #[cfg(feature = "minreq_http_sync")]
        return MethodSync::MinReqHttp;
        panic!("No method selected");
    }
}

#[cfg(feature = "sync")]
impl MethodSync {
    /// Fetches the result string from a URL using the selected method.
    pub fn fetch(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(match self {
            #[cfg(feature = "reqwest_sync")]
            MethodSync::Reqwest => reqwest::blocking::get(url)?.text()?,
            #[cfg(feature = "ureq_sync")]
            MethodSync::Ureq => ureq::get(url).call()?.into_string()?,
            #[cfg(feature = "httpreq_sync")]
            MethodSync::HttpReq => String::from_utf8(crate::functions::httpreq_get(url)?)?,
            #[cfg(feature = "minreq_http_sync")]
            MethodSync::MinReqHttp => String::from_utf8(minreq::get(url).send()?.into_bytes())?,
            #[cfg(feature = "minreq_sync")]
            MethodSync::MinReq => String::from_utf8(minreq::get(url).send()?.into_bytes())?,
            #[cfg(feature = "isahc_sync")]
            MethodSync::Isahc => {
                use isahc::ReadResponseExt;
                isahc::get(url)?.text()?
            }
            _ => panic!("No method selected"),
        })
    }
}

/// Request methods for `ClientAsync`.
///
/// ## Binary build times
///
/// Build times are tested on my local Arch Linux machine, with `--release` flag..
///
/// |Feature|Time(s)|Crate|
/// |---|---|---|
/// |None|6.9|-|
/// |reqwest_async|17.7|[reqwest](https://crates.io/crates/reqwest)|
/// |isahc_async|20.4|[isahc](https://crates.io/crates/isahc)|
#[cfg(feature = "async")]
#[derive(Clone, Copy)]
pub enum MethodAsync {
    /// Everyone knows about reqwest.
    #[cfg(feature = "reqwest_async")]
    Reqwest,
    /// Supports runtime other than tokio.
    #[cfg(feature = "isahc_async")]
    Isahc,
}

#[cfg(feature = "async")]
impl Default for MethodAsync {
    /// Returns the first enabled method. Panics if no methods are enabled in features.
    ///
    /// The ordering is as follows:
    /// 1. reqwest_async
    /// 2. isahc_async
    fn default() -> Self {
        #[cfg(feature = "reqwest_async")]
        return MethodAsync::Reqwest;
        #[cfg(feature = "isahc_async")]
        return MethodAsync::Isahc;
        panic!("No method selected");
    }
}

#[cfg(feature = "async")]
impl MethodAsync {
    /// Fetches the result string from a URL using the selected method.
    pub async fn fetch(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(match self {
            #[cfg(feature = "reqwest_async")]
            MethodAsync::Reqwest => reqwest::get(url).await?.text().await?,
            #[cfg(feature = "isahc_async")]
            MethodAsync::Isahc => {
                use isahc::AsyncReadResponseExt;
                isahc::get_async(url).await?.text().await?
            }
        })
    }
}

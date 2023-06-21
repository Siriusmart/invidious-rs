use crate::*;

#[test]
fn reqwest_sync() {
    ClientSync::default()
        .method(MethodSync::REQWEST)
        .video("FhhyqkbtaR4", None)
        .unwrap();
}

#[test]
fn ureq_sync() {
    ClientSync::default()
        .method(MethodSync::UREQ)
        .video("FhhyqkbtaR4", None)
        .unwrap();
}

#[test]
fn httpreq_sync() {
    ClientSync::default()
        .method(MethodSync::HTTPREQ)
        .video("FhhyqkbtaR4", None)
        .unwrap();
}

#[test]
fn minreq_http_sync() {
    ClientSync::default()
        .method(MethodSync::MINREQ_HTTP)
        .video("FhhyqkbtaR4", None)
        .unwrap();
}

#[test]
fn minreq_sync() {
    ClientSync::default()
        .method(MethodSync::MINREQ)
        .video("FhhyqkbtaR4", None)
        .unwrap();
}

#[test]
fn isahc_sync() {
    ClientSync::default()
        .method(MethodSync::ISAHC)
        .video("FhhyqkbtaR4", None)
        .unwrap();
}

#[tokio::test]
async fn reqwest_async() {
    ClientAsync::default()
        .method(MethodAsync::REQWEST)
        .video("FhhyqkbtaR4", None)
        .await
        .unwrap();
}

#[tokio::test]
async fn isahc_async() {
    ClientAsync::default()
        .method(MethodAsync::ISAHC)
        .video("FhhyqkbtaR4", None)
        .await
        .unwrap();
}

#[test]
fn custom_sync() {
    ClientSync::default()
        .custom_method(Box::new(sync_fetch))
        .video("FhhyqkbtaR4", None)
        .unwrap();
}

fn sync_fetch(url: &str) -> MethodReturn {
    Ok(reqwest::blocking::get(url)?.text()?)
}

#[tokio::test]
async fn custom_async() {
    ClientAsync::default()
        .custom_method(Box::new(async_fetch))
        .video("FhhyqkbtaR4", None)
        .await
        .unwrap();
}

async fn async_fetch(url: String) -> MethodReturn {
    Ok(reqwest::get(url).await?.text().await?)
}

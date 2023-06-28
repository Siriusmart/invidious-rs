use crate::*;

#[test]
fn reqwest_sync() {
    ClientSync::default()
        .method(MethodSync::Reqwest)
        .video("FhhyqkbtaR4", None)
        .unwrap();
}

#[test]
fn ureq_sync() {
    ClientSync::default()
        .method(MethodSync::Ureq)
        .video("FhhyqkbtaR4", None)
        .unwrap();
}

#[test]
fn httpreq_sync() {
    ClientSync::default()
        .method(MethodSync::HttpReq)
        .video("FhhyqkbtaR4", None)
        .unwrap();
}

#[test]
fn minreq_http_sync() {
    ClientSync::default()
        .method(MethodSync::MinReqHttp)
        .video("FhhyqkbtaR4", None)
        .unwrap();
}

#[test]
fn minreq_sync() {
    ClientSync::default()
        .method(MethodSync::MinReq)
        .video("FhhyqkbtaR4", None)
        .unwrap();
}

#[test]
fn isahc_sync() {
    ClientSync::default()
        .method(MethodSync::Isahc)
        .video("FhhyqkbtaR4", None)
        .unwrap();
}

#[tokio::test]
async fn reqwest_async() {
    ClientAsync::default()
        .method(MethodAsync::Reqwest)
        .video("FhhyqkbtaR4", None)
        .await
        .unwrap();
}

#[tokio::test]
async fn isahc_async() {
    ClientAsync::default()
        .method(MethodAsync::Isahc)
        .video("FhhyqkbtaR4", None)
        .await
        .unwrap();
}

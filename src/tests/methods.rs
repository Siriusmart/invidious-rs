use crate::*;

#[test]
fn reqwest_sync() {
    ClientSync::default()
        .method(MethodSync::ReqwestSync)
        .video("FhhyqkbtaR4", None)
        .unwrap();
}

#[test]
fn ureq_sync() {
    ClientSync::default()
        .method(MethodSync::UreqSync)
        .video("FhhyqkbtaR4", None)
        .unwrap();
}

#[test]
fn httpreq_sync() {
    ClientSync::default()
        .method(MethodSync::HttpReqSync)
        .video("FhhyqkbtaR4", None)
        .unwrap();
}

#[test]
fn minreq_http_sync() {
    ClientSync::default()
        .method(MethodSync::MinReqHttpSync)
        .video("FhhyqkbtaR4", None)
        .unwrap();
}

#[test]
fn minreq_sync() {
    ClientSync::default()
        .method(MethodSync::MinReqSync)
        .video("FhhyqkbtaR4", None)
        .unwrap();
}

#[test]
fn isahc_sync() {
    ClientSync::default()
        .method(MethodSync::IsahcSync)
        .video("FhhyqkbtaR4", None)
        .unwrap();
}

#[tokio::test]
async fn reqwest_async() {
    ClientAsync::default()
        .method(MethodAsync::ReqwestAsync)
        .video("FhhyqkbtaR4", None)
        .await
        .unwrap();
}

#[tokio::test]
async fn isahc_async() {
    ClientAsync::default()
        .method(MethodAsync::IsahcAsync)
        .video("FhhyqkbtaR4", None)
        .await
        .unwrap();
}

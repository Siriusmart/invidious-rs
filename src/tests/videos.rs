use crate::ClientAsyncTrait;

#[tokio::test]
async fn videos() {
    crate::ClientAsync::default()
        .method(crate::MethodAsync::Isahc)
        .video("FhhyqkbtaR4", None)
        .await
        .unwrap();
}

#[tokio::test]
async fn comments() {
    let client = crate::ClientAsync::default()
        .method(crate::MethodAsync::Isahc);
    let comments = client.comments("FhhyqkbtaR4", None).await.unwrap();
    client
        .comments(
            "FhhyqkbtaR4",
            Some(&format!("continuation={}", comments.continuation.unwrap())),
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn captions() {
    crate::ClientAsync::default()
        .method(crate::MethodAsync::Isahc)
        .captions("FhhyqkbtaR4", None)
        .await
        .unwrap();
}

#[tokio::test]
async fn upcoming() {
    crate::ClientAsync::default()
        .method(crate::MethodAsync::Isahc)
        .video("WyqKuHQ5CE8", None)
        .await
        .unwrap();
}

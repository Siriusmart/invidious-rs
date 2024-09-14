use crate::ClientAsyncTrait;

#[tokio::test]
async fn videos() {
    crate::ClientAsync::default()
        .method(crate::MethodAsync::Reqwest)
        .video("FhhyqkbtaR4", None)
        .await
        .unwrap();
}

#[tokio::test]
async fn comments() {
    let client = crate::ClientAsync::default().method(crate::MethodAsync::Reqwest);
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
        .method(crate::MethodAsync::Reqwest)
        .captions("FhhyqkbtaR4", None)
        .await
        .unwrap();
}

#[tokio::test]
async fn upcoming() {
    crate::ClientAsync::default()
        .method(crate::MethodAsync::Reqwest)
        .video("WyqKuHQ5CE8", None)
        .await
        .unwrap();
}

use crate::ClientAsyncTrait;

#[tokio::test]
async fn videos() {
    crate::ClientAsync::default()
        .method(crate::MethodAsync::Reqwest)
        .video("M3rGDpCG5FQ", None)
        .await
        .unwrap();
}

#[tokio::test]
async fn comments() {
    let client = crate::ClientAsync::default().method(crate::MethodAsync::Reqwest);
    let comments = client.comments("M3rGDpCG5FQ", None).await.unwrap();
    client
        .comments(
            "M3rGDpCG5FQ",
            Some(&format!("continuation={}", comments.continuation.unwrap())),
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn captions() {
    crate::ClientAsync::default()
        .method(crate::MethodAsync::Reqwest)
        .captions("M3rGDpCG5FQ", None)
        .await
        .unwrap();
}

// currently not working due to upstream limitations
#[tokio::test]
async fn upcoming() {
    crate::ClientAsync::default()
        .method(crate::MethodAsync::Reqwest)
        .video("SbWpqQGVfTA", None)
        .await
        .unwrap();
}

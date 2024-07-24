use crate::ClientAsyncTrait;

#[tokio::test]
async fn trending() {
    crate::ClientAsync::default()
        .method(crate::MethodAsync::Isahc)
        .trending(None).await.unwrap();
}

#[tokio::test]
async fn popular() {
    crate::ClientAsync::default()
        .method(crate::MethodAsync::Isahc)
        .popular(None).await.unwrap();
}

#[tokio::test]
async fn stats() {
    crate::ClientAsync::default()
        .method(crate::MethodAsync::Isahc)
        .stats(None).await.unwrap();
}

#[tokio::test]
async fn playlist() {
    crate::ClientAsync::default()
        .method(crate::MethodAsync::Isahc)
        .playlist("PLdgHTasZAjYaI2DUfqe70I82o9clPGyiO", None)
        .await
        .unwrap();
}

#[tokio::test]
async fn search() {
    crate::ClientAsync::default()
        .method(crate::MethodAsync::Isahc)
        .search(Some("q=testing"))
        .await
        .unwrap();
}

#[tokio::test]
async fn search_suggestions() {
    assert!(!crate::ClientAsync::default()
        .method(crate::MethodAsync::Isahc)
        .search_suggestions(Some("q=test"))
        .await
        .unwrap()
        .suggestions
        .is_empty())
}

use crate::ClientAsyncTrait;

#[tokio::test]
async fn channel() {
    crate::ClientAsync::default()
        .method(crate::MethodAsync::Isahc)
        .channel("UC7YOGHUfC1Tb6E4pudI9STA", None)
        .await
        .unwrap();
}

#[tokio::test]
async fn videos() {
    crate::ClientAsync::default()
        .method(crate::MethodAsync::Isahc)
        .channel_videos("UC7YOGHUfC1Tb6E4pudI9STA", None)
        .await
        .unwrap();
}

#[tokio::test]
async fn playlists() {
    crate::ClientAsync::default()
        .method(crate::MethodAsync::Isahc)
        .channel_playlists("UC7YOGHUfC1Tb6E4pudI9STA", None)
        .await
        .unwrap();
}

#[tokio::test]
async fn search() {
    crate::ClientAsync::default()
        .method(crate::MethodAsync::Isahc)
        .channel_search("UC7YOGHUfC1Tb6E4pudI9STA", Some("q=testing"))
        .await
        .unwrap();
}

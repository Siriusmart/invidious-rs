#[test]
fn channel() {
    crate::ClientSync::default()
        .channel("UC7YOGHUfC1Tb6E4pudI9STA", None)
        .unwrap();
}

#[test]
fn videos() {
    crate::ClientSync::default()
        .channel_videos("UC7YOGHUfC1Tb6E4pudI9STA", None)
        .unwrap();
}

#[test]
fn playlists() {
    crate::ClientSync::default()
        .channel_playlists("UC7YOGHUfC1Tb6E4pudI9STA", None)
        .unwrap();
}

#[test]
fn search() {
    crate::ClientSync::default()
        .channel_search("UC7YOGHUfC1Tb6E4pudI9STA", Some("q=testing"))
        .unwrap();
}

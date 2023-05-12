use crate::reqwest::blocking::Client;

#[test]
fn videos() {
    let client = Client::new(super::INSTANCE.to_string());
    let videos = ["d09-P9R5BRE"];

    for video in videos.iter() {
        client.video(video, None).unwrap();
    }
}

#[test]
fn comments() {
    let client = Client::new(super::INSTANCE.to_string());
    let videos = ["MSfD-QApDyU", "erEgovG9WBs"];

    for video in videos.iter() {
        client.comments(video, None).unwrap();
    }
}

#[test]
fn captions() {
    let client = Client::new(super::INSTANCE.to_string());
    let videos = ["u2hVK24UPWQ"];

    for video in videos.iter() {
        client.captions(video, None).unwrap();
    }
}

#[test]
fn annotations() {
    let client = Client::new(super::INSTANCE.to_string());
    let videos = ["u2hVK24UPWQ"];

    for video in videos.iter() {
        client.annotations(video, None).unwrap();
    }
}

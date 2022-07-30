use crate::reqwest::blocking::Client;

#[test]
fn videos() {
    let client = Client::new(String::from("https://vid.puffyan.us"));
    let videos = ["d09-P9R5BRE"];

    for video in videos.iter() {
        client.video(video, None).unwrap();
    }
}

#[test]
fn comments() {
    let client = Client::new(String::from("https://vid.puffyan.us"));
    let videos = ["MSfD-QApDyU"];

    for video in videos.iter() {
        client.comments(video, None).unwrap();
    }
}

#[test]
fn captions() {
    let client = Client::new(String::from("https://vid.puffyan.us"));
    let videos = ["u2hVK24UPWQ"];

    for video in videos.iter() {
        client.captions(video, None).unwrap();
    }
}

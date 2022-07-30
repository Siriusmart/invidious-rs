use crate::reqwest::blocking::{Client};

#[test]
pub fn channel() {
    let client = Client::new(String::from("https://vid.puffyan.us"));
    let channels = [
        "UC-lHJZR3Gqxm24_Vd_AJ5Yw",
    ];

    for channel in channels.iter() {
        client.channel(channel, None).unwrap();
        // functions::channel(&client.server, channel, None).unwrap();
    }
}
#[test]
pub fn videos() {
    let client = Client::new(String::from("https://vid.puffyan.us"));
    let channels = [
        "UC-lHJZR3Gqxm24_Vd_AJ5Yw",
    ];

    for channel in channels.iter() {
        client.channel_videos(channel, Some("sort_by=popular")).unwrap();
    }
}

#[test]
pub fn playlists() {
    let client = Client::new(String::from("https://vid.puffyan.us"));
    let channels = [
        "UC-lHJZR3Gqxm24_Vd_AJ5Yw",
    ];

    for channel in channels.iter() {
        client.channel_playlists(channel, None).unwrap();
    }
}

#[test]
pub fn comments() {
    let client = Client::new(String::from("https://vid.puffyan.us"));
    let channels = [
        "UC-lHJZR3Gqxm24_Vd_AJ5Yw",
    ];

    for channel in channels.iter() {
        client.channel_comments(channel, None).unwrap();
    }
}

#[test]
pub fn search() {
    let client = Client::new(String::from("https://vid.puffyan.us"));
    let channels = [
        "UC-lHJZR3Gqxm24_Vd_AJ5Yw",
    ];

    for channel in channels.iter() {
        client.channel_search(channel, Some("q=hello")).unwrap();
    }
}

use crate::blocking::Client;

#[test]
fn trending() {
    let client = Client::new(String::from("https://vid.puffyan.us"));
    let trending = client.trending(None).unwrap();

    for item in trending.videos[0..1].iter() {
        if let Err(e) = client.video(&item.videoId, None) {
            println!("{}", e);
            println!("{:#?}", item);
            panic!("failed to get video");
        }

        if let Err(e) = client.comments(&item.videoId, None) {
            println!("{}", e);
            println!("{:#?}", item);
            panic!("failed to get comments");
        }

        if let Err(e) = client.captions(&item.videoId, None) {
            println!("{}", e);
            println!("{:#?}", item);
            panic!("failed to get captions");
        }
    }
}

#[test]
fn popular() {
    let client = Client::new(String::from("https://vid.puffyan.us"));
    let popular = client.popular(None).unwrap();

    for item in popular.items[3..4].iter() {
        if let Err(e) = client.video(&item.videoId, None) {
            println!("{}", e);
            println!("{:#?}", item);
            panic!("failed to get video");
        }

        if let Err(e) = client.comments(&item.videoId, None) {
            println!("{}", e);
            println!("{:#?}", item);
            panic!("failed to get comments");
        }

        if let Err(e) = client.captions(&item.videoId, None) {
            println!("{}", e);
            println!("{:#?}", item);
            panic!("failed to get captions");
        }
    }
}

#[test]
fn stats() {
    let client = Client::new(String::from("https://vid.puffyan.us"));
    client.stats(None).unwrap();
}
#[test]
fn playlists() {
    let client = Client::new(String::from("https://vid.puffyan.us"));
    let playlists_to_check = [
        "PLdgHTasZAjYaI2DUfqe70I82o9clPGyiO",
        "PLdgHTasZAjYZlCXN9rTcX9LFOQ-RIrzCs",
    ];

    for playlist in playlists_to_check.iter() {
        client.playlist(playlist, None).unwrap();
    }
}

#[test]
fn search() {
    let client = Client::new(String::from("https://vid.puffyan.us"));
    let search_terms = ["rust"];

    let types = ["video", "playlist", "channel", "all"];

    for r#type in types {
        for search_term in search_terms.iter() {
            client
                .search(Some(&format!("q={}&type={}", search_term, r#type)))
                .unwrap();
        }
    }
}


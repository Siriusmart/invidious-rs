use crate::curl::blocking::Client;

#[test]
pub fn curl() {
    let client = Client::new(String::from("https://vid.puffyan.us"));
    client.stats(None).unwrap();
}

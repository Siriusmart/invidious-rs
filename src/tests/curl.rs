use crate::curl::blocking::Client;

#[test]
pub fn curl() {
    let client = Client::new(super::INSTANCE.to_string());
    client.stats(None).unwrap();
}

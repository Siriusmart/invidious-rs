use crate::ClientSyncTrait;

#[test]
fn videos() {
    crate::ClientSync::default()
        .video("FhhyqkbtaR4", None)
        .unwrap();
}

#[test]
fn comments() {
    let client = crate::ClientSync::default();
    let comments = client.comments("FhhyqkbtaR4", None).unwrap();
    client
        .comments(
            "FhhyqkbtaR4",
            Some(&format!("continuation={}", comments.continuation.unwrap())),
        )
        .unwrap();
}

#[test]
fn captions() {
    crate::ClientSync::default()
        .captions("FhhyqkbtaR4", None)
        .unwrap();
}

#[test]
fn upcoming() {
    crate::ClientSync::default()
        .video("WyqKuHQ5CE8", None)
        .unwrap();
}

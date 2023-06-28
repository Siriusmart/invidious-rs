use crate::ClientSyncTrait;

#[test]
fn videos() {
    crate::ClientSync::default()
        .video("FhhyqkbtaR4", None)
        .unwrap();
}

#[test]
fn comments() {
    crate::ClientSync::default()
        .comments("FhhyqkbtaR4", None)
        .unwrap();
}

#[test]
fn captions() {
    crate::ClientSync::default()
        .captions("FhhyqkbtaR4", None)
        .unwrap();
}

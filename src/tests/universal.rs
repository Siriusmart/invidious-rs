#[test]
fn trending() {
    crate::ClientSync::default().trending(None).unwrap();
}

#[test]
fn popular() {
    crate::ClientSync::default().popular(None).unwrap();
}

#[test]
fn stats() {
    crate::ClientSync::default().stats(None).unwrap();
}

#[test]
fn playlist() {
    crate::ClientSync::default()
        .playlist("PLdgHTasZAjYaI2DUfqe70I82o9clPGyiO", None)
        .unwrap();
}

#[test]
fn search() {
    crate::ClientSync::default()
        .search(Some("q=testing"))
        .unwrap();
}

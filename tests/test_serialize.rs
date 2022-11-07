use dfm::dotinfo::DotFileInstallInfo;

#[test]
fn test_serialize() {
    let raw_str = std::fs::read_to_string("./tests/example_info.hjson").unwrap();
    let hjson: DotFileInstallInfo = deser_hjson::from_str(&raw_str).unwrap();
    dbg!(hjson);
}

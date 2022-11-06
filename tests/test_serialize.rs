use dfm::dotinfo::DotFileInstallInfo;

#[test]
fn test_serialize() {
    let raw_str = std::fs::read_to_string("./tests/example_info.json").unwrap();
    let json: DotFileInstallInfo = serde_json::from_str(&raw_str).unwrap();
    dbg!(json);
}

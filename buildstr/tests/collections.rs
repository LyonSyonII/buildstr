use buildstr::BuildStr;

#[test]
fn btreemap() {
    let map = std::collections::BTreeMap::from_iter([
        ("a".to_string(), 1),
        ("b".to_string(), 2),
        ("c".to_string(), 3),
        ("d".to_string(), 4),
    ]);
    assert_eq!(map.to_build_string(), "std::collections::BTreeMap::from_iter([(String::from(\"a\"),1i32),(String::from(\"b\"),2i32),(String::from(\"c\"),3i32),(String::from(\"d\"),4i32),])");
}

use buildstr::BuildStr;

#[test]
fn dir_builder() {
    assert_eq!(
        ::std::fs::DirBuilder::new().to_build_string(),
        "::std::fs::DirBuilder::new()"
    )
}
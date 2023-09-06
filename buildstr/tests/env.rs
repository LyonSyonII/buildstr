use buildstr::BuildStr;

#[test]
fn join_paths_error() {
    assert_eq!(
        ::std::env::join_paths([":\""]).unwrap_err().to_build_string(),
        r#"::std::env::join_paths([":\""]).unwrap_err()"#
    )
}
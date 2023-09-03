use buildstr::BuildStr;

#[test]
fn pending() {
    let p = ::core::future::pending::<i32>();
    assert_eq!(p.to_build_string(), "::core::future::pending::<i32>()");
}

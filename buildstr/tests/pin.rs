use buildstr::BuildStr;

#[test]
fn pin() {
    let v = Box::pin(5);
    assert_eq!(v.to_build_string(), "std::pin::Pin::new(Box::new(5i32))");
}
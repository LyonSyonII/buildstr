use buildstr::BuildStr;

#[test]
fn pin() {
    let mut v = Box::pin(5);
    assert_eq!(v.to_build_string(), "std::pin::Pin::new(Box::new(5i32))");
    let v: std::pin::Pin<&mut &mut std::pin::Pin<Box<i32>>> = std::pin::pin!(&mut v);
    assert_eq!(
        v.to_build_string(),
        "std::pin::Pin::new(&mut &mut std::pin::Pin::new(Box::new(5i32)))"
    );
}

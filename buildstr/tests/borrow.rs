use buildstr::BuildStr;

#[test]
fn cow() {
    assert_eq!(::std::borrow::Cow::Borrowed("hello").to_build_string(), "::std::borrow::Cow::Borrowed(\"hello\")");
    assert_eq!(::std::borrow::Cow::Owned::<str>(::std::string::String::from("hello")).to_build_string(), "::std::borrow::Cow::Owned::<str>(::std::string::String::from(\"hello\"))");
}
use buildstr::BuildStr;
use ::std::borrow::Cow;

#[test]
fn cow() {
    let _ = ::std::borrow::Cow::Borrowed("hello");
    assert_eq!(Cow::Borrowed("hello").to_build_string(), "::std::borrow::Cow::Borrowed(\"hello\")");
    assert_eq!(Cow::Owned::<str>(String::from("hello")).to_build_string(), "::std::borrow::Cow::Owned::<str>(::std::string::String::from(\"hello\"))");
}
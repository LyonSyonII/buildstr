use buildstr::BuildStr;

#[test]
fn cow() {
    assert_eq!(
        ::std::borrow::Cow::Borrowed("hello").to_build_string(),
        "::std::borrow::Cow::Borrowed::<str>(\"hello\")"
    );
    assert_eq!(
        ::std::borrow::Cow::Owned::<str>(::std::string::String::from("hello")).to_build_string(),
        "::std::borrow::Cow::Owned::<str>(::std::string::String::from(\"hello\"))"
    );

    assert_eq!(
        ::std::borrow::Cow::Borrowed(&5).to_build_string(),
        "::std::borrow::Cow::Borrowed::<i32>(&5i32)"
    );
    assert_eq!(
        ::std::borrow::Cow::Owned::<i32>(5).to_build_string(),
        "::std::borrow::Cow::Owned::<i32>(5i32)"
    );

    assert_eq!(
        ::std::borrow::Cow::Borrowed::<[i32]>(&[1i32, 2i32, 3i32,]).to_build_string(),
        "::std::borrow::Cow::Borrowed::<[i32]>(&[1i32,2i32,3i32,])"
    );
    assert_eq!(
        ::std::borrow::Cow::Owned::<[i32]>(::std::vec::Vec::from_iter([1i32, 2i32, 3i32,]))
            .to_build_string(),
        "::std::borrow::Cow::Owned::<[i32]>(::std::vec::Vec::from_iter([1i32,2i32,3i32,]))"
    );
}

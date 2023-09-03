use buildstr::BuildStr;

#[test]
fn char_try_from_error() {
    assert_eq!(::std::primitive::char::try_from(::std::primitive::u32::MAX).unwrap_err().to_build_string(), "::std::primitive::char::try_from(::std::primitive::u32::MAX).unwrap_err()");
}

#[test]
fn decode_utf16() {
    assert_eq!(::std::primitive::char::decode_utf16([0xD800,0xDC00,]).to_build_string(), "::std::primitive::char::decode_utf16([55296u16,56320u16,])");
}
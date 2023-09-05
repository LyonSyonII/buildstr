use buildstr::BuildStr;

#[test]
fn char() {
    assert_eq!('c'.to_build_string(), "\'c\'");
    assert_eq!('\t'.to_build_string(), "\'\\t\'");
}

#[test]
fn char_try_from_error() {
    assert_eq!(
        ::std::primitive::char::try_from(::std::primitive::u32::MAX)
            .unwrap_err()
            .to_build_string(),
        "::std::primitive::char::try_from(::std::primitive::u32::MAX).unwrap_err()"
    );
}

#[test]
fn decode_utf16() {
    assert_eq!(
        ::std::primitive::char::decode_utf16([0xD800, 0xDC00,]).to_build_string(),
        "::std::primitive::char::decode_utf16([55296u16,56320u16,])"
    );
    assert_eq!(
        ::std::primitive::char::decode_utf16([]).to_build_string(),
        "::std::primitive::char::decode_utf16([])"
    );
}

#[test]
fn decode_utf16error() {
    assert_eq!(
        ::std::primitive::char::decode_utf16([0xDD1E])
            .next()
            .unwrap()
            .unwrap_err()
            .to_build_string(),
        "::std::primitive::char::decode_utf16([56606u16]).next().unwrap().unwrap_err()"
    );
}

#[test]
fn escape_debug() {
    assert_eq!(
        ::std::primitive::char::escape_debug('\t').to_build_string(),
        "::std::primitive::char::escape_debug('\\t')"
    );
}

#[test]
fn escape_default() {
    assert_eq!(
        ::std::primitive::char::escape_default('\t').to_build_string(),
        "::std::primitive::char::escape_default('\\t')"
    );
}

#[test]
fn escape_unicode() {
    assert_eq!(
        ::std::primitive::char::escape_unicode('\t').to_build_string(),
        "::std::primitive::char::escape_unicode('\\t')"
    );
}

#[test]
fn parse_char_error() {
    assert_eq!(
        <::std::primitive::char as ::std::str::FromStr>::from_str("")
            .unwrap_err()
            .to_build_string(),
        "<::std::primitive::char as ::std::str::FromStr>::from_str(\"\").unwrap_err()"
    );
    assert_eq!(
        <::std::primitive::char as ::std::str::FromStr>::from_str("abcdef")
            .unwrap_err()
            .to_build_string(),
        "<::std::primitive::char as ::std::str::FromStr>::from_str(\"aa\").unwrap_err()"
    );
}

#[test]
fn to_lowercase() {
    assert_eq!(
        ::std::primitive::char::to_lowercase('A').to_build_string(),
        "::std::primitive::char::to_lowercase('a')"
    );
    assert_eq!(
        'İ'.to_lowercase().to_build_string(),
        "::std::primitive::char::to_lowercase('İ')"
    )
}

#[test]
fn to_uppercase() {
    assert_eq!(
        ::std::primitive::char::to_uppercase('a').to_build_string(),
        "::std::primitive::char::to_uppercase('A')"
    );
    assert_eq!(
        'ß'.to_uppercase().to_build_string(),
        "::std::primitive::char::to_uppercase('ß')"
    )
}
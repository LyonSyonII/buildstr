use std::error::Error;

use buildstr::BuildStr;

#[test]
fn error() {
    assert_eq!(
        (&"5a".parse::<u8>().unwrap_err() as &dyn Error).to_build_string(),
        r#"<&::std::primitive::str as ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error>>>::into("invalid digit found in string").as_ref()"#
    );
    assert_eq!(
          (<&::std::primitive::str as ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error>>>::into("invalid digit found in string").as_ref()).to_build_string(),
        r#"<&::std::primitive::str as ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error>>>::into("invalid digit found in string").as_ref()"#
    )
}
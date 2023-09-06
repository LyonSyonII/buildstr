use buildstr::BuildStr;

#[test]
fn as_ref() {
    assert_eq!(
        (&"" as &dyn ::std::convert::AsRef<str>).to_build_string(), 
        "&\"\" as &dyn ::std::convert::AsRef<str>"
    );
    assert_eq!(
        (&String::new() as &dyn AsRef<str>).to_build_string(),
        "&\"\" as &dyn ::std::convert::AsRef<str>"
    );
    assert_eq!(
        (&&[] as &dyn ::std::convert::AsRef<[u8]>).to_build_string(),
        "&&[] as &dyn ::std::convert::AsRef<[u8]>"
    );
    assert_eq!(
        (&&[1u8, 2u8, 3u8,] as &dyn ::std::convert::AsRef<[u8]>).to_build_string(),
        "&&[1u8,2u8,3u8,] as &dyn ::std::convert::AsRef<[u8]>"
    );
    assert_eq!(
        (&Vec::new() as &dyn AsRef<[u8]>).to_build_string(),
        "&&[] as &dyn ::std::convert::AsRef<[u8]>"
    );
    assert_eq!(
        (&vec![&"a" as &dyn AsRef<str>, &"b", &"c",] as &dyn AsRef<[&dyn AsRef<str>]>).to_build_string(),
        "&&[&\"a\" as &dyn ::std::convert::AsRef<str>,&\"b\" as &dyn ::std::convert::AsRef<str>,&\"c\" as &dyn ::std::convert::AsRef<str>,] as &dyn ::std::convert::AsRef<[&dyn core::convert::AsRef<str>]>"
    );
    assert_eq!(
        (&&[&"a" as &dyn ::std::convert::AsRef<str>,&"b" as &dyn ::std::convert::AsRef<str>,&"c" as &dyn ::std::convert::AsRef<str>,] as &dyn ::std::convert::AsRef<[&dyn core::convert::AsRef<str>]>).to_build_string(),
        "&&[&\"a\" as &dyn ::std::convert::AsRef<str>,&\"b\" as &dyn ::std::convert::AsRef<str>,&\"c\" as &dyn ::std::convert::AsRef<str>,] as &dyn ::std::convert::AsRef<[&dyn core::convert::AsRef<str>]>"
    );
}
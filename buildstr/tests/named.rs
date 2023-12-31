pub use buildstr::BuildStr;

#[test]
pub fn simple() {
    #[derive(BuildStr)]
    struct Person {
        name: String,
        surname: &'static str,
        initial: char,
        age: u8,
        is_human: bool,
        money: f64,
    }

    let p = Person {
        name: "Dolphin".into(),
        surname: "Cute",
        initial: 'D',
        age: 14,
        is_human: false,
        money: 5.5,
    };

    assert_eq!(p.to_build_string(), "Person{name: ::std::string::String::from(\"Dolphin\"),surname: \"Cute\",initial: 'D',age: 14u8,is_human: false,money: 5.5f64,}");
}

#[test]
pub fn lists() {
    #[derive(BuildStr)]
    struct Person<'a> {
        name: String,
        age: u8,
        cars: Vec<&'static str>,
        degrees: &'a [String],
    }

    let p = Person {
        name: "Potato".into(),
        age: 42,
        cars: vec!["Toyota", "Ford", "Chevy"],
        degrees: &["BSc".into(), "MSc".into(), "PhD".into()],
    };

    let _ = Person {
        name: ::std::string::String::from("Potato"),
        age: 42u8,
        cars: ::std::vec::Vec::from_iter(["Toyota", "Ford", "Chevy"]),
        degrees: &[
            ::std::string::String::from("BSc"),
            ::std::string::String::from("MSc"),
            ::std::string::String::from("PhD"),
        ],
    };
    assert_eq!(p.to_build_string(), "Person{name: ::std::string::String::from(\"Potato\"),age: 42u8,cars: ::std::vec::Vec::from_iter([\"Toyota\",\"Ford\",\"Chevy\",]),degrees: &[::std::string::String::from(\"BSc\"),::std::string::String::from(\"MSc\"),::std::string::String::from(\"PhD\"),],}");
}

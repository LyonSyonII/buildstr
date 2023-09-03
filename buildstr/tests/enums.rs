use buildstr::BuildStr;

#[test]
fn unit() {
    #[allow(unused)]
    #[derive(BuildStr)]
    enum Fruits {
        Apple,
        Banana,
        Pear,
    }

    assert_eq!(Fruits::Apple.to_build_string(), "Fruits::Apple");
}

#[test]
fn unnamed() {
    #[derive(BuildStr)]
    enum Color {
        Grayscale(u8),
        Rgb(u8, u8, u8),
        Rgb128(&'static u128, &'static u128, &'static u128),
    }

    assert_eq!(
        Color::Grayscale(1).to_build_string(),
        "Color::Grayscale(1u8,)"
    );
    assert_eq!(
        Color::Rgb(255, 255, 255).to_build_string(),
        "Color::Rgb(255u8,255u8,255u8,)"
    );
    assert_eq!(
        Color::Rgb128(&255, &255, &255).to_build_string(),
        "Color::Rgb128(&255u128,&255u128,&255u128,)"
    )
}

#[test]
fn named() {
    #[allow(unused)]
    #[derive(BuildStr)]
    enum Animals {
        Dog { name: String, age: u8 },
        Cat { name: String, age: u8 },
    }
    assert_eq!(
        Animals::Dog {
            name: "Fido".to_string(),
            age: 3,
        }
        .to_build_string(),
        "Animals::Dog{name: ::std::string::String::from(\"Fido\"),age: 3u8,}"
    );
}

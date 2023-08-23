pub use buildstr::derive::BuildStr;
pub use buildstr::BuildStr;

trait BuildStr2 {
    fn to_build_string(&self) -> String;
}

#[test]
fn simple() {
    impl BuildStr2 for (u8, u8, u8, u8, u8, u8, u8, u8) {
        fn to_build_string(&self) -> String {
            format!(
                "({}u8,{}u8,{}u8,{}u8,{}u8,{}u8,{}u8,{}u8)",
                self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7
            )
        }
    }

    #[derive(BuildStr)]
    struct A((u8, u8, u8, u8, u8, u8, u8, u8), u64);

    assert_eq!(
        A((1, 2, 3, 4, 5, 6, 7, 8), 7).to_build_string(),
        "A((1u8,2u8,3u8,4u8,5u8,6u8,7u8,8u8),7u64,)"
    );
}

#[test]
fn compound() {
    trait BuildStr2 {
        fn to_build_string(&self) -> String;
    }

    impl BuildStr2 for Vec<std::io::Error> {
        fn to_build_string(&self) -> String {
            "".into()
        }
    }

    #[derive(BuildStr)]
    struct A(Vec<std::io::Error>);
}

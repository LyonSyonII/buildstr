use buildstr::BuildStr;

#[test]
fn simple() {
    #[derive(BuildStr)]
    struct Generic<T> {
        value: T,
    }
    let g = Generic {
        value: 5
    };
    assert_eq!(g.to_build_string(), "Generic{value:5i32,}");
}

#[test]
fn lifetime() {
    #[derive(BuildStr)]
    struct Generic<'a, T> {
        value: &'a T
    }
    let g = Generic {
        value: &5
    };
    assert_eq!(g.to_build_string(), "Generic{value:&5i32,}");
}
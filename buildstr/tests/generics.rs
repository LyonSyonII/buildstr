use buildstr::BuildStr;

#[test]
fn simple() {
    #[derive(BuildStr)]
    struct Generic<T> {
        value: T,
    }
    let g = Generic { value: 5 };
    assert_eq!(g.to_build_string(), "Generic{value:5i32,}");
}

#[test]
fn lifetime() {
    #[derive(BuildStr)]
    struct Generic<'a, T> {
        value: &'a T,
    }
    let g = Generic { value: &5 };
    assert_eq!(g.to_build_string(), "Generic{value:&5i32,}");
}

#[test]
fn multiple() {
    #[derive(BuildStr)]
    struct Generic<'a, A, B, C> {
        a: A,
        b: &'a B,
        c: C,
    }
    let g = Generic { a: 5, b: &6, c: 7 };
    assert_eq!(g.to_build_string(), "Generic{a:5i32,b:&6i32,c:7i32,}");
}

#[test]
fn constraint() {
    #[derive(BuildStr)]
    struct Generic<T: Clone>(T);
    let g = Generic(5);
    assert_eq!(g.to_build_string(), "Generic(5i32,)");
}

#[test]
fn r#where() {
    #[derive(BuildStr)]
    struct Generic<T>(T)
    where
        T: Clone;
    let g = Generic(5);
    assert_eq!(g.to_build_string(), "Generic(5i32,)");
}

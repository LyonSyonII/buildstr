use buildstr::BuildStr;

#[allow(clippy::needless_borrow)]
#[test]
fn simple() {
    let o = 5;
    let r = &5;
    assert_eq!(o.to_build_string(), "5i32");
    assert_eq!(r.to_build_string(), "5i32");

    assert_eq!(BuildStr::to_build_string(&o), "5i32");
    assert_eq!(BuildStr::to_build_string(&r), "&5i32");

    assert_eq!((&o).to_build_string(), "5i32");
    assert_eq!((&r).to_build_string(), "&5i32");
}

#[test]
fn r#struct() {
    #[derive(BuildStr)]
    struct Ref<'a, T> {
        ptr: &'a T,
        owned: T,
    }

    let s = String::from("patata");
    let r = Ref {
        ptr: &s,
        owned: s.clone(),
    };
    assert_eq!(r.to_build_string(), "Ref{ptr:&::std::string::String::from(\"patata\"),owned:::std::string::String::from(\"patata\"),}");
}

#[test]
fn r#enum() {
    #[derive(BuildStr)]
    enum Ref<'a, T> {
        Ptr(&'a T),
        Owned(T),
    }

    let s = String::from("patata");
    let r = Ref::Ptr(&s);
    assert_eq!(
        r.to_build_string(),
        "Ref::Ptr(&::std::string::String::from(\"patata\"),)"
    );
    let r = Ref::Owned(s.clone());
    assert_eq!(
        r.to_build_string(),
        "Ref::Owned(::std::string::String::from(\"patata\"),)"
    );
}
// (\w*\.?\w+)\.to_build_string\(\)

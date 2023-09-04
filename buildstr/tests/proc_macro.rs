#[test]
fn proc_macro() {
    use buildstr::BuildStr;

    #[derive(BuildStr)]
    enum Fruits {
        Apple,
        Banana(usize),
        Pear { quantity: usize },
        Melon(char),
    }

    let fruits = vec![
        Fruits::Apple,
        Fruits::Banana(2),
        Fruits::Pear { quantity: 3 },
    ];
    assert_eq!(
        fruits.to_build_string(),
        "::std::vec::Vec::from_iter([Fruits::Apple,Fruits::Banana(2usize,),Fruits::Pear{quantity: 3usize,},])"
    );
    assert_eq!(
        fruits.to_build_tokens().to_string(),
        ":: std :: vec :: Vec :: from_iter ([Fruits :: Apple , Fruits :: Banana (2usize ,) , Fruits :: Pear { quantity : 3usize , } ,])"
    );

    let melon = Fruits::Melon('\t');
    assert_eq!(melon.to_build_string(), "Fruits::Melon('\\t',)");
    assert_eq!(
        melon.to_build_tokens().to_string(),
        "Fruits :: Melon ('\\t' ,)"
    );
}

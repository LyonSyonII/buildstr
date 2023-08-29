#[test]
fn proc_macro() {
    use buildstr::BuildStr;

    #[derive(BuildStr)]
    enum Fruits {
        Apple,
        Banana(usize),
        Pear { quantity: usize },
    }

    let fruits = vec![
        Fruits::Apple,
        Fruits::Banana(2),
        Fruits::Pear { quantity: 3 },
    ];
    assert_eq!(
        fruits.to_build_string(),
        "vec![Fruits::Apple,Fruits::Banana(2usize,),Fruits::Pear{quantity:3usize,},]"
    );
    assert_eq!(
        fruits.to_build_tokens().to_string(),
        "vec ! [Fruits :: Apple , Fruits :: Banana (2usize ,) , Fruits :: Pear { quantity : 3usize , } ,]"
    )
}

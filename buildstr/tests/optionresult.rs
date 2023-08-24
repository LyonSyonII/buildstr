use buildstr::BuildStr;

#[test]
fn option() {
    #[derive(BuildStr)]
    struct Options {
        background: Option<(u8, u8, u8)>,
    }
}

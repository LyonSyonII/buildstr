use buildstr::BuildStr;

#[test]
fn arguments() {
    assert_eq!(format_args!("{} {} {}", 1, 2, 3).to_build_string(), "core::format_args!(\"1 2 3\")");
}
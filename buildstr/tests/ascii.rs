use core::ascii::escape_default as e;

use buildstr::BuildStr;

#[test]
fn escape_default() {
    assert_eq!(e(b'\t').to_build_string(), "::std::ascii::escape_default(9)");
    assert_eq!(b'\t', 9);
    assert_eq!(e(b'\t').collect::<Vec<_>>(), e(9).collect::<Vec<_>>());

    assert_eq!(e(b'\x9d').to_build_string(), "::std::ascii::escape_default(157)"); 
    assert_eq!(b'\x9d', 157);
    assert_eq!(e(b'\x9d').collect::<Vec<_>>(), e(157).collect::<Vec<_>>());
}

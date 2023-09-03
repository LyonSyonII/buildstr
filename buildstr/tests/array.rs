use buildstr::BuildStr;

#[test]
fn into_iter() {
    let _ = ([1i32, 2i32, 3i32] as [i32; 3]).into_iter();
    assert_eq!(
        [1, 2, 3].into_iter().to_build_string(),
        "([1i32,2i32,3i32,]as[i32;3]).into_iter()"
    );
}

#[test]
fn try_from_slice_error() {
    let err: Result<[(); 1], std::array::TryFromSliceError> =
        core::convert::TryInto::<[(); 1]>::try_into(&[] as &[()]);
    assert_eq!(err.to_build_string(), "core::result::Result::Err(core::convert::TryInto::<[();1]>::try_into(&[]as&[()]).unwrap_err())");
}

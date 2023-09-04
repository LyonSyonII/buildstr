use crate::BuildStr;

impl<T: BuildStr, const N: usize> BuildStr for ::std::array::IntoIter<T, N> {
    fn to_build_string(&self) -> String {
        let iter = self.as_slice();
        let v = buildstr::array_to_build_string!(iter);
        format!("([{v}]as[{};{N}]).into_iter()", ::std::any::type_name::<T>())
    }
}

impl BuildStr for ::std::array::TryFromSliceError {
    fn to_build_string(&self) -> String {
        "::core::convert::TryInto::<[();1]>::try_into(&[]as&[()]).unwrap_err()".into()
    }
}
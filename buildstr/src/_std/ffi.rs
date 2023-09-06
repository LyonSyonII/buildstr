use crate::BuildStr;

impl BuildStr for ::std::ffi::CStr {
    fn to_build_string(&self) -> String {
        format!(
            "::std::ffi::CStr::from_bytes_with_nul({}).unwrap()",
            self.to_bytes_with_nul().to_build_string()
        )
    }
}

impl BuildStr for ::std::ffi::CString {
    fn to_build_string(&self) -> String {
        let b = buildstr::array_to_build_string!(self.to_bytes());
        format!("::std::ffi::CString::new([{b}]).unwrap()")
    }
}

impl BuildStr for &::std::ffi::OsStr {
    fn to_build_string(&self) -> String {
        format!("::std::ffi::OsStr::new({self:?})")
    }
}

impl BuildStr for ::std::ffi::OsString {
    fn to_build_string(&self) -> String {
        format!("::std::ffi::OsString::from({self:?})")
    }
}

impl BuildStr for std::ffi::FromBytesWithNulError {
    fn to_build_string(&self) -> String {
        // 1. data provided contains an interior nul byte at byte pos X
        // 2. data provided is not nul terminated
        let s = self
            .to_string()
            .split_once("pos ")
            .map(|(_, s)| format!("{}\0a", "a".repeat(s.parse().unwrap())))
            .unwrap_or_default();
        format!("::std::ffi::CStr::from_bytes_with_nul(\"{s}\").unwrap_err()")
    }
}

impl BuildStr for std::ffi::FromVecWithNulError {
    fn to_build_string(&self) -> String {
        let v = self.as_bytes().to_vec().to_build_string();
        format!("::std::ffi::CString::from_vec_with_nul({v}).unwrap_err()")
    }
}

impl BuildStr for std::ffi::IntoStringError {
    fn to_build_string(&self) -> String {
        let s = self.clone().into_cstring().to_build_string();
        format!("{s}.into_string().unwrap_err()")
    }
}

impl BuildStr for std::ffi::NulError {
    fn to_build_string(&self) -> String {
        let v = buildstr::array_to_build_string!(self.clone().into_vec());
        format!("::std::ffi::CString::new([{v}]).unwrap_err()")
    }
}
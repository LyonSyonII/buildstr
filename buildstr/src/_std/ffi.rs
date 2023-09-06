use crate::BuildStr;

impl BuildStr for ::core::ffi::CStr {
    fn to_build_string(&self) -> String {
        format!("::std::ffi::CStr::from_bytes_with_nul({}).unwrap()", self.to_bytes_with_nul().to_build_string())
    }
}

impl BuildStr for ::std::ffi::CString {
    fn to_build_string(&self) -> String {
        format!("::std::ffi::CString::new({}).unwrap()", self.to_bytes_with_nul().to_build_string())
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
        let s = self.to_string();
        let s = match s.as_str() {
            "data provided contains an interior nul byte" => "\0a",
            "data provided is not nul terminated" => "",
            _ => todo!("{s:?} case is not handled. Please, open an issue at https://github.com/lyonsyonii/buildstr.")
        };
        format!("::std::ffi::CStr::from_bytes_with_nul(\"{s}\").unwrap_err()")
    }
}
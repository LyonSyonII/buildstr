use crate::BuildStr;

impl BuildStr for core::ffi::CStr {
    fn to_build_string(&self) -> String {
        format!("core::ffi::CStr::from_bytes_with_nul({}).unwrap()", self.to_bytes_with_nul().to_build_string())
    }
}
impl BuildStr for std::ffi::CString {
    fn to_build_string(&self) -> String {
        format!("core::ffi::CString::new({}).unwrap()", self.to_bytes_with_nul().to_build_string())
    }
}
impl BuildStr for &std::ffi::OsStr {
    fn to_build_string(&self) -> String {
        format!("core::ffi::OsStr::new({:?})", self)
    }
}
impl BuildStr for std::ffi::OsString {
    fn to_build_string(&self) -> String {
        format!("core::ffi::OsString::from({:?})", self)
    }
}
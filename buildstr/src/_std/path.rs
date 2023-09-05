use buildstr::BuildStr;

impl BuildStr for &::std::path::Path {
    fn to_build_string(&self) -> String {
        format!("::std::path::Path::new({self:?})")
    }
}
impl BuildStr for ::std::path::PathBuf {
    fn to_build_string(&self) -> String {
        format!("::std::path::PathBuf::from({self:?})")
    }
}
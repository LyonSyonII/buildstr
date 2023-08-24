use crate::BuildStr;

impl BuildStr for &str {
    fn to_build_string(&self) -> String {
        format!("\"{self}\"")
    }
}
impl BuildStr for String {
    fn to_build_string(&self) -> String {
        format!("std::string::String::from({:?})", self)
    }
}

use crate::BuildStr;


impl BuildStr for ::std::string::String {
    fn to_build_string(&self) -> String {
        format!("::std::string::String::from({:?})", self)
    }
}
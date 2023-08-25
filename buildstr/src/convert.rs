use crate::BuildStr;

impl BuildStr for core::convert::Infallible {
    fn to_build_string(&self) -> String {
        "core::convert::Infallible".to_string()
    }
}
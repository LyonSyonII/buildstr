use crate::BuildStr;

impl BuildStr for ::core::ops::RangeFull {
    fn to_build_string(&self) -> String {
        "::core::ops::RangeFull".into()
    }
}
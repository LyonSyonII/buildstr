use crate::BuildStr;

impl BuildStr for core::time::Duration {
    fn to_build_string(&self) -> String {
        format!("core::time::Duration::new({}, {})", self.as_secs(), self.subsec_nanos())
    }
}
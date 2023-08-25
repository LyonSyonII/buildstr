use crate::BuildStr;

impl BuildStr for core::cmp::Ordering {
    fn to_build_string(&self) -> String {
        match self {
            std::cmp::Ordering::Less => "core::cmp::Ordering::Less",
            std::cmp::Ordering::Equal => "core::cmp::Ordering::Equal",
            std::cmp::Ordering::Greater => "core::cmp::Ordering::Greater",
        }.to_string()
    }
}
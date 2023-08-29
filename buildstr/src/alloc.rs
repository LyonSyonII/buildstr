use crate::BuildStr;

impl BuildStr for std::alloc::Layout {
    fn to_build_string(&self) -> String {
        let size = self.size();
        let align = self.align();
        // SAFETY: The Layout is valid because the original one is
        format!("unsafe{{Self::from_size_align_unchecked({size},{align})}}")
    }
}

impl BuildStr for std::alloc::System {
    fn to_build_string(&self) -> String {
        "std::alloc::System".into()
    }
}
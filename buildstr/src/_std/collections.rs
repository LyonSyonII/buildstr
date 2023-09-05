use crate::BuildStr;

impl BuildStr for std::collections::TryReserveError {
    fn to_build_string(&self) -> String {
        let s = self.to_string();
        match s.as_str() {
            "memory allocation failed because the computed capacity exceeded the collection's maximum" => "::std::vec::Vec::<u8>::with_capacity(1).try_reserve_exact(::std::primitive::usize::MAX).unwrap_err()",
            "memory allocation failed because the memory allocator returned an error" => todo!("AllocError kind not implemented, please open an issue at https://github.com/lyonsyonii/buildstr."),
            _ => todo!("{s:?} is not handled. Please, open an issue at https://github.com/lyonsyonii/buildstr.")
        }.into()
    }
}
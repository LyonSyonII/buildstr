use crate::BuildStr;

#[cfg(target_arch = "x86_64")]
mod x86_64 {
    use super::*;

    impl BuildStr for std::arch::x86_64::CpuidResult {
        fn to_build_string(&self) -> String {
            let eax = self.eax;
            let ebx = self.ebx;
            let ecx = self.ecx;
            let edx = self.edx;
            format!("std::arch::x86_64::CpuidResult{{eax:{eax},ebx:{ebx},ecx:{ecx},edx:{edx}}}")
        }
    }
}

#[cfg(target_arch = "x86")]
mod x86 {
    use super::*;

    impl BuildStr for std::arch::x86::CpuidResult {
        fn to_build_string(&self) -> String {
            let eax = self.eax;
            let ebx = self.ebx;
            let ecx = self.ecx;
            let edx = self.edx;
            format!("std::arch::x86::CpuidResult{{eax:{eax},ebx:{ebx},ecx:{ecx},edx:{edx}}}")
        }
    }
}
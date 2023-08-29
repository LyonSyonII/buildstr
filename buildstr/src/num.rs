use crate::BuildStr;

impl BuildStr for core::num::FpCategory {
    fn to_build_string(&self) -> String {
        match self {
            core::num::FpCategory::Nan => "core::num::FpCategory::Nan",
            core::num::FpCategory::Infinite => "core::num::FpCategory::Infinite",
            core::num::FpCategory::Zero => "core::num::FpCategory::Zero",
            core::num::FpCategory::Subnormal => "core::num::FpCategory::Subnormal",
            core::num::FpCategory::Normal => "core::num::FpCategory::Normal",
        }.into()
    }
}
impl BuildStr for core::num::IntErrorKind {
    fn to_build_string(&self) -> String {
        match self {
            core::num::IntErrorKind::Empty => "core::num::IntErrorKind::Empty",
            core::num::IntErrorKind::InvalidDigit => "core::num::IntErrorKind::InvalidDigit",
            core::num::IntErrorKind::PosOverflow => "core::num::IntErrorKind::PosOverflow",
            core::num::IntErrorKind::NegOverflow => "core::num::IntErrorKind::NegOverflow",
            core::num::IntErrorKind::Zero => "core::num::IntErrorKind::Zero",
            _ => unreachable!("IntErrorKind should not have another value"),
        }.into()
    }
}
impl BuildStr for core::num::ParseIntError {
    fn to_build_string(&self) -> String {
        format!("core::num::ParseIntError::new({})", self.kind().to_build_string())
    }
}

impl BuildStr for core::num::NonZeroU8 {
    fn to_build_string(&self) -> String {
        format!("core::num::NonZeroU8::new({})", self.get())
    }
}
impl BuildStr for core::num::NonZeroU16 {
    fn to_build_string(&self) -> String {
        format!("core::num::NonZeroU16::new({})", self.get())
    }
}
impl BuildStr for core::num::NonZeroU32 {
    fn to_build_string(&self) -> String {
        format!("core::num::NonZeroU32::new({})", self.get())
    }
}
impl BuildStr for core::num::NonZeroU64 {
    fn to_build_string(&self) -> String {
        format!("core::num::NonZeroU64::new({})", self.get())
    }
}
impl BuildStr for core::num::NonZeroU128 {
    fn to_build_string(&self) -> String {
        format!("core::num::NonZeroU128::new({})", self.get())
    }
}
impl BuildStr for core::num::NonZeroUsize {
    fn to_build_string(&self) -> String {
        format!("core::num::NonZeroUsize::new({})", self.get())
    }
}
impl BuildStr for core::num::NonZeroI8 {
    fn to_build_string(&self) -> String {
        format!("core::num::NonZeroI8::new({})", self.get())
    }
}
impl BuildStr for core::num::NonZeroI16 {
    fn to_build_string(&self) -> String {
        format!("core::num::NonZeroI16::new({})", self.get())
    }
}
impl BuildStr for core::num::NonZeroI32 {
    fn to_build_string(&self) -> String {
        format!("core::num::NonZeroI32::new({})", self.get())
    }
}
impl BuildStr for core::num::NonZeroI64 {
    fn to_build_string(&self) -> String {
        format!("core::num::NonZeroI64::new({})", self.get())
    }
}
impl BuildStr for core::num::NonZeroI128 {
    fn to_build_string(&self) -> String {
        format!("core::num::NonZeroI128::new({})", self.get())
    }
}
impl BuildStr for core::num::NonZeroIsize {
    fn to_build_string(&self) -> String {
        format!("core::num::NonZeroIsize::new({})", self.get())
    }
}
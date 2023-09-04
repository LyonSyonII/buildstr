use crate::BuildStr;

impl BuildStr for bool {
    fn to_build_string(&self) -> String {
        format!("{self:?}")
    }
}
impl BuildStr for char {
    fn to_build_string(&self) -> String {
        format!("{self:?}")
    }
}
impl BuildStr for &str {
    fn to_build_string(&self) -> String {
        format!("\"{self}\"")
    }
}
impl BuildStr for u8 {
    fn to_build_string(&self) -> String {
        format!("{self}u8")
    }
}
impl BuildStr for u16 {
    fn to_build_string(&self) -> String {
        format!("{self}u16")
    }
}
impl BuildStr for u32 {
    fn to_build_string(&self) -> String {
        format!("{self}u32")
    }
}
impl BuildStr for u64 {
    fn to_build_string(&self) -> String {
        format!("{self}u64")
    }
}
impl BuildStr for u128 {
    fn to_build_string(&self) -> String {
        format!("{self}u128")
    }
}
impl BuildStr for usize {
    fn to_build_string(&self) -> String {
        format!("{self}usize")
    }
}
impl BuildStr for i8 {
    fn to_build_string(&self) -> String {
        format!("{self}i8")
    }
}
impl BuildStr for i16 {
    fn to_build_string(&self) -> String {
        format!("{self}i16")
    }
}
impl BuildStr for i32 {
    fn to_build_string(&self) -> String {
        format!("{self}i32")
    }
}
impl BuildStr for i64 {
    fn to_build_string(&self) -> String {
        format!("{self}i64")
    }
}
impl BuildStr for i128 {
    fn to_build_string(&self) -> String {
        format!("{self}i128")
    }
}
impl BuildStr for isize {
    fn to_build_string(&self) -> String {
        format!("{self}isize")
    }
}
impl BuildStr for f32 {
    fn to_build_string(&self) -> String {
        format!("{self}f32")
    }
}
impl BuildStr for f64 {
    fn to_build_string(&self) -> String {
        format!("{self}f64")
    }
}
impl BuildStr for () {
    fn to_build_string(&self) -> String {
        ::std::string::String::from("()")
    }
}
use crate::BuildStr;

impl BuildStr for bool {
    fn to_build_string(&self) -> String {
        format!("{:?}", self)
    }
}
impl BuildStr for char {
    fn to_build_string(&self) -> String {
        format!("{:?}", self)
    }
}
impl BuildStr for &str {
    fn to_build_string(&self) -> String {
        format!("\"{self}\"")
    }
}
impl BuildStr for u8 {
    fn to_build_string(&self) -> String {
        format!("{}u8", self)
    }
}
impl BuildStr for u16 {
    fn to_build_string(&self) -> String {
        format!("{}u16", self)
    }
}
impl BuildStr for u32 {
    fn to_build_string(&self) -> String {
        format!("{}u32", self)
    }
}
impl BuildStr for u64 {
    fn to_build_string(&self) -> String {
        format!("{}u64", self)
    }
}
impl BuildStr for u128 {
    fn to_build_string(&self) -> String {
        format!("{}u128", self)
    }
}
impl BuildStr for usize {
    fn to_build_string(&self) -> String {
        format!("{}usize", self)
    }
}
impl BuildStr for i8 {
    fn to_build_string(&self) -> String {
        format!("{}i8", self)
    }
}
impl BuildStr for i16 {
    fn to_build_string(&self) -> String {
        format!("{}i16", self)
    }
}
impl BuildStr for i32 {
    fn to_build_string(&self) -> String {
        format!("{}i32", self)
    }
}
impl BuildStr for i64 {
    fn to_build_string(&self) -> String {
        format!("{}i64", self)
    }
}
impl BuildStr for i128 {
    fn to_build_string(&self) -> String {
        format!("{}i128", self)
    }
}
impl BuildStr for isize {
    fn to_build_string(&self) -> String {
        format!("{}isize", self)
    }
}
impl BuildStr for f32 {
    fn to_build_string(&self) -> String {
        format!("{}f32", self)
    }
}
impl BuildStr for f64 {
    fn to_build_string(&self) -> String {
        format!("{}f64", self)
    }
}
impl BuildStr for () {
    fn to_build_string(&self) -> String {
        ::std::string::String::from("()")
    }
}
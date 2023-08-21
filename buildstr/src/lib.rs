pub mod derive {
    pub use buildstr_derive::BuildStr;
}

pub trait BuildStr {
    fn to_build_string(&self) -> String;
}

impl BuildStr for &str {
    fn to_build_string(&self) -> String {
        format!("\"{self}\"")
    }
}

impl BuildStr for String {
    fn to_build_string(&self) -> String {
        format!("String::from({:?})", self)
    }
}

impl BuildStr for char {
    fn to_build_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl BuildStr for bool {
    fn to_build_string(&self) -> String {
        format!("{:?}", self)
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
        String::from("()")
    }
}

impl<T> BuildStr for Option<T> where T: BuildStr {
    fn to_build_string(&self) -> String {
        match self {
            Some(s) => format!("Some({})", s.to_build_string()),
            None => String::from("None"),
        }
    }
}

impl<R, E> BuildStr for Result<R, E> where R: BuildStr, E: BuildStr {
    fn to_build_string(&self) -> String {
        match self {
            Ok(s) => format!("Ok({})", s.to_build_string()),
            Err(s) => format!("Err({})", s.to_build_string()),
        }
    }
}

impl<T> BuildStr for Box<T> where T: BuildStr {
    fn to_build_string(&self) -> String {
        format!("Box::new({})", self.as_ref().to_build_string())
    }
}

impl<T> BuildStr for std::rc::Rc<T> where T: BuildStr {
    fn to_build_string(&self) -> String {
        format!("Rc::new({})", self.as_ref().to_build_string())
    }
}

impl<T> BuildStr for Vec<T> where T: BuildStr {
    fn to_build_string(&self) -> String {
        let v = self.iter().map(|x| x.to_build_string()).collect::<Vec<_>>();
        format!("vec![{}]", v.join(","))
    }
}

impl<T> BuildStr for &[T] where T: BuildStr {
    fn to_build_string(&self) -> String {
        let v = self.iter().map(|x| x.to_build_string()).collect::<Vec<_>>();
        format!("&[{}]", v.join(","))
    }
}

impl<T> BuildStr for &mut [T] where T: BuildStr {
    fn to_build_string(&self) -> String {
        let v = self.iter().map(|x| x.to_build_string()).collect::<Vec<_>>();
        format!("&mut [{}]", v.join(","))
    }
}

impl<T, const N: usize> BuildStr for [T; N] where T: BuildStr {
    fn to_build_string(&self) -> String {
        let v = self.iter().map(|x| x.to_build_string()).collect::<Vec<_>>();
        format!("[{}]", v.join(","))
    }
}

impl<A, B> BuildStr for (A, B) where A: BuildStr, B: BuildStr {
    fn to_build_string(&self) -> String {
        format!("({}, {})", self.0.to_build_string(), self.1.to_build_string())
    }
}

impl<A, B, C> BuildStr for (A, B, C) where A: BuildStr, B: BuildStr, C: BuildStr {
    fn to_build_string(&self) -> String {
        format!("({}, {}, {})", self.0.to_build_string(), self.1.to_build_string(), self.2.to_build_string())
    }
}

impl<A, B, C, D> BuildStr for (A, B, C, D) where A: BuildStr, B: BuildStr, C: BuildStr, D: BuildStr {
    fn to_build_string(&self) -> String {
        format!("({}, {}, {}, {})", self.0.to_build_string(), self.1.to_build_string(), self.2.to_build_string(), self.3.to_build_string())
    }
}

impl<A, B, C, D, E> BuildStr for (A, B, C, D, E) where A: BuildStr, B: BuildStr, C: BuildStr, D: BuildStr, E: BuildStr {
    fn to_build_string(&self) -> String {
        format!("({}, {}, {}, {}, {})", self.0.to_build_string(), self.1.to_build_string(), self.2.to_build_string(), self.3.to_build_string(), self.4.to_build_string())
    }
}

impl<A, B, C, D, E, F> BuildStr for (A, B, C, D, E, F) where A: BuildStr, B: BuildStr, C: BuildStr, D: BuildStr, E: BuildStr, F: BuildStr {
    fn to_build_string(&self) -> String {
        format!("({}, {}, {}, {}, {}, {})", self.0.to_build_string(), self.1.to_build_string(), self.2.to_build_string(), self.3.to_build_string(), self.4.to_build_string(), self.5.to_build_string())
    }
}

impl<T> BuildStr for &T where T: BuildStr {
    fn to_build_string(&self) -> String {
        format!("&{}", BuildStr::to_build_string(*self))
    }
}

impl<T> BuildStr for &mut T where T: BuildStr {
    fn to_build_string(&self) -> String {
        format!("&mut {}", BuildStr::to_build_string(*self))
    }
}
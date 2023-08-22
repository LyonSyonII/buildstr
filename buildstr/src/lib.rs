#[cfg(feature = "derive")]
pub mod derive {
    pub use buildstr_derive::BuildStr;
}

#[cfg(feature = "pretty")]
pub trait Pretty {
    fn to_pretty_build_string(&self) -> String;
}
#[cfg(feature = "pretty")]
impl<T> Pretty for T
where
    T: BuildStr,
{
    fn to_pretty_build_string(&self) -> String {
        let expr = syn::parse_str(&self.to_build_string()).unwrap();
        prettier_please::unparse_expr(&expr)
    }
}

pub trait BuildStr {
    fn to_build_string(&self) -> String;
}

#[cfg(feature = "string")]
impl BuildStr for &str {
    fn to_build_string(&self) -> String {
        format!("\"{self}\"")
    }
}
#[cfg(feature = "string")]
impl BuildStr for String {
    fn to_build_string(&self) -> String {
        format!("String::from({:?})", self)
    }
}
#[cfg(feature = "primitives")]
impl BuildStr for char {
    fn to_build_string(&self) -> String {
        format!("{:?}", self)
    }
}
#[cfg(feature = "primitives")]
impl BuildStr for bool {
    fn to_build_string(&self) -> String {
        format!("{:?}", self)
    }
}
#[cfg(feature = "primitives")]
impl BuildStr for u8 {
    fn to_build_string(&self) -> String {
        format!("{}u8", self)
    }
}
#[cfg(feature = "primitives")]
impl BuildStr for u16 {
    fn to_build_string(&self) -> String {
        format!("{}u16", self)
    }
}
#[cfg(feature = "primitives")]
impl BuildStr for u32 {
    fn to_build_string(&self) -> String {
        format!("{}u32", self)
    }
}
#[cfg(feature = "primitives")]
impl BuildStr for u64 {
    fn to_build_string(&self) -> String {
        format!("{}u64", self)
    }
}
#[cfg(feature = "primitives")]
impl BuildStr for usize {
    fn to_build_string(&self) -> String {
        format!("{}usize", self)
    }
}
#[cfg(feature = "primitives")]
impl BuildStr for i8 {
    fn to_build_string(&self) -> String {
        format!("{}i8", self)
    }
}
#[cfg(feature = "primitives")]
impl BuildStr for i16 {
    fn to_build_string(&self) -> String {
        format!("{}i16", self)
    }
}
#[cfg(feature = "primitives")]
impl BuildStr for i32 {
    fn to_build_string(&self) -> String {
        format!("{}i32", self)
    }
}
#[cfg(feature = "primitives")]
impl BuildStr for i64 {
    fn to_build_string(&self) -> String {
        format!("{}i64", self)
    }
}
#[cfg(feature = "primitives")]
impl BuildStr for isize {
    fn to_build_string(&self) -> String {
        format!("{}isize", self)
    }
}
#[cfg(feature = "primitives")]
impl BuildStr for f32 {
    fn to_build_string(&self) -> String {
        format!("{}f32", self)
    }
}
#[cfg(feature = "primitives")]
impl BuildStr for f64 {
    fn to_build_string(&self) -> String {
        format!("{}f64", self)
    }
}
#[cfg(feature = "primitives")]
impl BuildStr for () {
    fn to_build_string(&self) -> String {
        String::from("()")
    }
}
#[cfg(feature = "option")]
impl<T> BuildStr for Option<T>
where
    T: BuildStr,
{
    fn to_build_string(&self) -> String {
        match self {
            Some(s) => format!("Some({})", s.to_build_string()),
            None => String::from("None"),
        }
    }
}
#[cfg(feature = "result")]
impl<R, E> BuildStr for Result<R, E>
where
    R: BuildStr,
    E: BuildStr,
{
    fn to_build_string(&self) -> String {
        match self {
            Ok(s) => format!("Ok({})", s.to_build_string()),
            Err(s) => format!("Err({})", s.to_build_string()),
        }
    }
}
#[cfg(feature = "box")]
impl<T> BuildStr for Box<T>
where
    T: BuildStr,
{
    fn to_build_string(&self) -> String {
        format!("Box::new({})", self.as_ref().to_build_string())
    }
}
#[cfg(feature = "rc")]
impl<T> BuildStr for std::rc::Rc<T>
where
    T: BuildStr,
{
    fn to_build_string(&self) -> String {
        format!("Rc::new({})", self.as_ref().to_build_string())
    }
}
#[cfg(feature = "rc")]
impl<T> BuildStr for std::rc::Weak<T>
where
    T: BuildStr,
{
    fn to_build_string(&self) -> String {
        match self.upgrade() {
            Some(s) => format!(
                "std::rc::Rc::downgrade(&std::rc::Rc::new({}))",
                s.as_ref().to_build_string()
            ),
            None => String::from("std::rc::Weak::new()"),
        }
    }
}
#[cfg(feature = "vec")]
impl<T> BuildStr for Vec<T>
where
    T: BuildStr,
{
    fn to_build_string(&self) -> String {
        format!("vec![{}]", array_to_build_string(self))
    }
}
#[cfg(feature = "array")]
impl<T> BuildStr for &[T]
where
    T: BuildStr,
{
    fn to_build_string(&self) -> String {
        format!("&[{}]", array_to_build_string(self.iter()))
    }
}
#[cfg(feature = "array")]
impl<T> BuildStr for &mut [T]
where
    T: BuildStr,
{
    fn to_build_string(&self) -> String {
        let v = self.iter().map(|x| x.to_build_string()).collect::<Vec<_>>();
        format!("&mut [{}]", v.join(","))
    }
}
#[cfg(feature = "array")]
impl<T, const N: usize> BuildStr for [T; N]
where
    T: BuildStr,
{
    fn to_build_string(&self) -> String {
        let v = self.iter().map(|x| x.to_build_string()).collect::<Vec<_>>();
        format!("[{}]", v.join(","))
    }
}
#[cfg(feature = "tuple")]
impl<A, B> BuildStr for (A, B)
where
    A: BuildStr,
    B: BuildStr,
{
    fn to_build_string(&self) -> String {
        format!(
            "({}, {})",
            self.0.to_build_string(),
            self.1.to_build_string()
        )
    }
}
#[cfg(feature = "tuple")]
impl<A, B, C> BuildStr for (A, B, C)
where
    A: BuildStr,
    B: BuildStr,
    C: BuildStr,
{
    fn to_build_string(&self) -> String {
        format!(
            "({}, {}, {})",
            self.0.to_build_string(),
            self.1.to_build_string(),
            self.2.to_build_string()
        )
    }
}
#[cfg(feature = "tuple")]
impl<A, B, C, D> BuildStr for (A, B, C, D)
where
    A: BuildStr,
    B: BuildStr,
    C: BuildStr,
    D: BuildStr,
{
    fn to_build_string(&self) -> String {
        format!(
            "({}, {}, {}, {})",
            self.0.to_build_string(),
            self.1.to_build_string(),
            self.2.to_build_string(),
            self.3.to_build_string()
        )
    }
}
#[cfg(feature = "tuple")]
impl<A, B, C, D, E> BuildStr for (A, B, C, D, E)
where
    A: BuildStr,
    B: BuildStr,
    C: BuildStr,
    D: BuildStr,
    E: BuildStr,
{
    fn to_build_string(&self) -> String {
        format!(
            "({}, {}, {}, {}, {})",
            self.0.to_build_string(),
            self.1.to_build_string(),
            self.2.to_build_string(),
            self.3.to_build_string(),
            self.4.to_build_string()
        )
    }
}
#[cfg(feature = "tuple")]
impl<A, B, C, D, E, F> BuildStr for (A, B, C, D, E, F)
where
    A: BuildStr,
    B: BuildStr,
    C: BuildStr,
    D: BuildStr,
    E: BuildStr,
    F: BuildStr,
{
    fn to_build_string(&self) -> String {
        format!(
            "({}, {}, {}, {}, {}, {})",
            self.0.to_build_string(),
            self.1.to_build_string(),
            self.2.to_build_string(),
            self.3.to_build_string(),
            self.4.to_build_string(),
            self.5.to_build_string()
        )
    }
}
#[cfg(feature = "reference")]
impl<T> BuildStr for &T
where
    T: BuildStr,
{
    fn to_build_string(&self) -> String {
        format!("&{}", BuildStr::to_build_string(*self))
    }
}
#[cfg(feature = "reference")]
impl<T> BuildStr for &mut T
where
    T: BuildStr,
{
    fn to_build_string(&self) -> String {
        format!("&mut {}", BuildStr::to_build_string(*self))
    }
}
#[cfg(feature = "btree")]
impl<K, V> BuildStr for std::collections::BTreeMap<K, V>
where
    K: BuildStr + core::cmp::Ord,
    V: BuildStr,
{
    fn to_build_string(&self) -> String {
        format!(
            "std::collections::BTreeMap::from_iter([{}])",
            map_to_build_string(self)
        )
    }
}

fn array_to_build_string<'a, T: BuildStr + 'a>(array: impl IntoIterator<Item = &'a T>) -> String {
    let mut s = String::new();
    let array = array.into_iter().map(|x| x.to_build_string());
    for a in array {
        s.push_str(&a);
        s.push(',');
    }
    s
}

fn map_to_build_string<'a, K: BuildStr + 'a, V: BuildStr + 'a>(
    map: impl IntoIterator<Item = (&'a K, &'a V)>,
) -> String {
    let mut s = String::new();
    let map = map
        .into_iter()
        .map(|(k, v)| format!("({},{})", k.to_build_string(), v.to_build_string()));
    for m in map {
        s.push_str(&m);
        s.push(',');
    }
    s
}

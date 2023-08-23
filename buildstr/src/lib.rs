#[cfg(feature = "derive")]
pub mod derive {
    pub use buildstr_derive::BuildStr;
}

pub use buildstr_derive::impl_buildstr;

#[cfg(feature = "primitives")]
mod primitives;

#[cfg(feature = "string")]
mod string;

extern crate self as buildstr;

#[macro_export]
macro_rules! array_to_build_string {
    ($array:ident) => {{
        let mut s = String::new();
        let array = $array.iter().map(|x| x.to_build_string());
        for a in array {
            s.push_str(&a);
            s.push(',');
        }
        s
    }};
}

#[macro_export]
macro_rules! map_to_build_string {
    ($map:ident) => {{
        let mut s = String::new();
        let map = $map
            .iter()
            .map(|(k, v)| format!("({},{})", k.to_build_string(), v.to_build_string()));
        for m in map {
            s.push_str(&m);
            s.push(',');
        }
        s
    }};
}

#[cfg(feature = "pretty")]
#[doc(hidden)]
pub fn __pretty(code: String) -> String {
    let expr = syn::parse_str(&code).unwrap();
    prettier_please::unparse_expr(&expr)
}

impl_buildstr!(BuildStr);

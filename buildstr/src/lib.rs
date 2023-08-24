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

/// Transforms an iterable of a single value to an array-like sequence without the enclosing brackets.
/// 
/// # Examples
/// ```
/// use buildstr::array_to_build_string;
/// 
/// let array = array_to_build_string!(&[1, 2, 3]);
/// assert_eq!(array, "1,2,3");
#[macro_export]
macro_rules! array_to_build_string {
    ($array:expr) => {
        {
            // TODO: Test fails, why?
            let mut s = String::new();
            let array = $array.iter().map(|x| x.to_build_string());
            for a in array {
                s.push_str(&a);
                s.push(',');
            }
            s
        }
    };
}

/// Transforms an iterable of a tuple of size two to an array-like sequence without the enclosing brackets.
/// 
/// # Examples
/// ```
/// use buildstr::map_to_build_string;
/// 
/// let map = [("one", 1), ("two", 2), ("three, 3)];
/// assert_eq!(map_to_build_string!(map), "(one,1),(two,2),(three,3)");
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
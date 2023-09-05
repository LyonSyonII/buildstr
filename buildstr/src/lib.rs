#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::needless_borrow)]

#[cfg(feature = "derive")]
pub use buildstr_derive::BuildStr;

pub use buildstr_derive::impl_buildstr;

extern crate self as buildstr;

/// Transforms an iterable of a single value to an array-like sequence without the enclosing brackets.
///
/// # Examples
/// ```
/// use buildstr::BuildStr;
/// use buildstr::array_to_build_string;
///
/// let array = array_to_build_string!(&[1, 2, 3]);
/// assert_eq!(array, "1i32,2i32,3i32,");
/// ```
#[macro_export]
macro_rules! array_to_build_string {
    ($array:expr) => {{
        let mut s = String::new();
        let array = $array;
        let array = array.iter().map(|x| x.to_build_string());
        for a in array {
            s.push_str(&a);
            s.push(',');
        }
        s
    }};
}

/// Transforms an iterable of a tuple of size two to an array-like sequence without the enclosing brackets.
///
/// # Examples
/// ```
/// use buildstr::map_to_build_string;
/// use buildstr::BuildStr;
///
/// let map = [("one", 1), ("two", 2), ("three", 3)];
/// assert_eq!(map_to_build_string!(map), "(\"one\",1i32),(\"two\",2i32),(\"three\",3i32),");
/// ```
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

impl_buildstr!(BuildStr);

pub mod __private;
mod _std;
#[cfg(feature = "pretty")]
#[doc(hidden)]
pub fn __pretty(code: impl AsRef<str>) -> String {
    let expr = syn::parse_str(code.as_ref()).unwrap();
    prettier_please::unparse_expr(&expr)
}

#[cfg(feature = "proc-macro")]
pub use proc_macro2::TokenStream;

#[cfg(feature = "proc-macro")]
pub fn __str_to_tokens(s: String) -> TokenStream {
    s.parse().unwrap()
}

pub(crate) mod unescape;
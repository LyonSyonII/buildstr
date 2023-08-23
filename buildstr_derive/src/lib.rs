use proc_macro::TokenStream;

#[cfg(feature = "derive")]
fn add_trait_bounds(mut generics: syn::Generics) -> syn::Generics {
    for param in &mut generics.params {
        if let syn::GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(syn::parse_quote!(buildstr::BuildStr));
        }
    }
    generics
}

#[cfg(feature = "derive")]
#[proc_macro_derive(BuildStr)]
pub fn buildstr(input: TokenStream) -> TokenStream {
    use syn::spanned::Spanned;
    use quote::{quote, quote_spanned};

    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = input.ident;
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let body = match input.data {
        syn::Data::Struct(ref s) => match s.fields {
            syn::Fields::Named(ref fields) => {
                let fields = fields.named.iter().map(|field| {
                    let name = &field.ident;
                    quote::quote_spanned! {field.span()=>
                        format!("{}:{},", stringify!(#name), self.#name.to_build_string())
                    }
                });

                quote::quote! {
                    let mut s = format!("{}{{", stringify!(#name));
                    #(s.push_str(&#fields);)*
                    s.push('}');
                    s
                }
            }
            syn::Fields::Unnamed(ref fields) => {
                let fields = fields.unnamed.iter().enumerate().map(|(i, field)| {
                    let name = syn::Index::from(i);
                    quote_spanned! {field.span()=>
                        format!("{},", self.#name.to_build_string())
                    }
                });
                quote! {
                    let mut s = format!("{}(", stringify!(#name));
                    #(s.push_str(&#fields);)*
                    s.push(')');
                    s
                }
            }
            syn::Fields::Unit => {
                quote! {
                    impl BuildStr for #name {
                        fn to_build_string(&self) -> String {
                            format!("{}", #name)
                        }
                    }
                }
            }
        },
        syn::Data::Enum(e) => {
            todo!()
        }
        syn::Data::Union(u) => todo!(),
    };

    quote! {
        impl #impl_generics BuildStr for #name #ty_generics #where_clause {
            fn to_build_string(&self) -> String {
                #body
            }
        }
    }
    .into()
}

/// Creates a local implementation of the `BuildStr` trait, to allow implementing it on foreign types.<br>
/// This works because the [`BuildStr`] derive macro is unhygienic, allowing mixed trait implementations.
/// 
/// The macro will create implementations for all generic data structures based on the specified feature flags,<br>
/// meaning that you will not need to implement `Vec<T>` manually, only `T`.
/// 
/// # Examples
/// ```
/// use buildstr::{impl_buildstr, BuildStr, derive::BuildStr};
/// 
/// impl_buildstr!(BuildStr2);
/// 
/// // num_bigint::BigInt does not implement `BuildStr`, so we need to implement it manually
/// impl BuildStr2 for num_bigint::BigInt {
///     fn to_build_string(&self) -> String {
///         format!("num_bigint::BigInt::from_str({})", self.to_string())
///     }
/// }
/// 
/// #[derive(BuildStr)]
/// struct Bank {
///     name: String,
///     accounts: Vec<Account>,
///     total_assets: num_bigint::BigInt,
/// }
/// 
/// #[derive(BuildStr)]
/// struct Account {
///     account_number: String,
///     balance_history: Vec<num_bigint::BigInt>,
///     current_balance: num_bigint::BigInt,
/// }
/// ```
#[proc_macro]
pub fn impl_buildstr(input: TokenStream) -> TokenStream {
    let name = input.to_string();
    let mut out = stringify! {
        pub trait BuildStr {
            fn to_build_string(&self) -> String;
        }
    }.to_owned();

    macro_rules! add_impls {
        ( $($feature:literal => $name:ident)* ) => {
            $(
            #[cfg(feature = $feature)]
            out.push_str($name());
            )*
        }
    }

    add_impls! {
        "pretty" => pretty
        "option" => option
        "result" => result
        "box" => r#box
        "rc" => rc
        "array" => array
        "vec" => vec
        "tuple" => tuple
        "reference" => reference
        "btree" => btree
    }
    
    out.replace("BuildStr", &name).parse().unwrap()
}

macro_rules! impls {
    ( $(fn $name:ident() $code:block)* ) => {
        $(
            #[allow(unused)]
            fn $name() -> &'static str {
                let s = stringify! {
                    $code
                };
                &s[1..s.len()-1]
            }
        )*
    }
}

fn pretty() -> &'static str {
    stringify! {
        pub trait Pretty {
            fn to_pretty_build_string(&self) -> String;
        }
        
        impl<T: BuildStr> Pretty for T {
            fn to_pretty_build_string(&self) -> String {
                buildstr::__pretty(self.to_build_string())
            }
        }
        
    }
}

impls! {

fn option() {
    impl<T: BuildStr> BuildStr for Option<T> {
        fn to_build_string(&self) -> String {
            match self {
                Some(s) => format!("Some({})", s.to_build_string()),
                None => String::from("None"),
            }
        }
    }
}

fn result() {
    impl<T, E> BuildStr for Result<T, E> where T: BuildStr, E: BuildStr {
        fn to_build_string(&self) -> String {
            match self {
                Ok(s) => format!("Ok({})", s.to_build_string()),
                Err(s) => format!("Err({})", s.to_build_string()),
            }
        }
    }
}

fn r#box() {
    impl<T: BuildStr> BuildStr for Box<T> {
        fn to_build_string(&self) -> String {
            format!("Box::new({})", self.as_ref().to_build_string())
        }
    }
}

fn rc() {
    impl<T: BuildStr> BuildStr for std::rc::Rc<T> {
        fn to_build_string(&self) -> String {
            format!("Rc::new({})", self.as_ref().to_build_string())
        }
    }
    impl<T: BuildStr> BuildStr for std::rc::Weak<T> {
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
}

fn array() {
    impl<T: BuildStr> BuildStr for &[T] {
        fn to_build_string(&self) -> String {
            format!("&[{}]", buildstr::array_to_build_string!(self))
        }
    }
    impl<T: BuildStr> BuildStr for &mut [T] {
        fn to_build_string(&self) -> String {
            format!("&mut [{}]", buildstr::array_to_build_string!(self))
        }
    }
    impl<T: BuildStr, const N: usize> BuildStr for [T; N] {
        fn to_build_string(&self) -> String {
            format!("[{}]", buildstr::array_to_build_string!(self))
        }
    }
}

fn vec() {
    impl<T: BuildStr> BuildStr for Vec<T> {
        fn to_build_string(&self) -> String {
            format!("vec![{}]", buildstr::array_to_build_string!(self))
        }
    }
}

fn tuple() {
    impl<A, B> BuildStr for (A, B) where A: BuildStr, B: BuildStr {
        fn to_build_string(&self) -> String {
            format!(
                "({}, {})",
                self.0.to_build_string(),
                self.1.to_build_string()
            )
        }
    }
    impl<A, B, C> BuildStr for (A, B, C)
    where A: BuildStr, B: BuildStr, C: BuildStr {
        fn to_build_string(&self) -> String {
            format!(
                "({}, {}, {})",
                self.0.to_build_string(),
                self.1.to_build_string(),
                self.2.to_build_string()
            )
        }
    }
    impl<A, B, C, D> BuildStr for (A, B, C, D)
    where A: BuildStr, B: BuildStr, C: BuildStr, D: BuildStr {
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
    impl<A, B, C, D, E> BuildStr for (A, B, C, D, E)
    where A: BuildStr, B: BuildStr, C: BuildStr, D: BuildStr, E: BuildStr {
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
    impl<A, B, C, D, E, F> BuildStr for (A, B, C, D, E, F)
    where A: BuildStr, B: BuildStr, C: BuildStr, D: BuildStr, E: BuildStr, F: BuildStr {
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
}

fn reference() {
    impl<T: BuildStr> BuildStr for &T {
        fn to_build_string(&self) -> String {
            format!("&{}", BuildStr::to_build_string(*self))
        }
    }
    impl<T: BuildStr> BuildStr for &mut T {
        fn to_build_string(&self) -> String {
            format!("&mut {}", BuildStr::to_build_string(*self))
        }
    }
}

fn btree() {
    impl<K, V> BuildStr for std::collections::BTreeMap<K, V> where K: BuildStr + core::cmp::Ord, V: BuildStr {
        fn to_build_string(&self) -> String {
            format!(
                "std::collections::BTreeMap::from_iter([{}])",
                buildstr::map_to_build_string!(self)
            )
        }
    }
    impl<T: BuildStr> BuildStr for std::collections::BTreeSet<T> {
        fn to_build_string(&self) -> String {
            format!(
                "std::collections::BTreeSet::from_iter([{}])",
                buildstr::array_to_build_string!(self)
            )
        }
    }
}

}


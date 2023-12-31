use proc_macro::TokenStream;

#[cfg(feature = "derive")]
use quote::{quote, quote_spanned};
#[cfg(feature = "derive")]
use syn::spanned::Spanned;

/// Derives the `BuildStr` trait for a `struct` or `enum`.
///
/// All types in the struct must have an associated function called `to_build_string`.<br>
/// This function is already implemented for all common std types.
///
/// *If the function is not available, check that you have enabled the corresponding feature.*
///
/// To implement it for foreign types, use [`impl_buildstr!`](crate::impl_buildstr).
///
/// # Examples
/// ```
/// use buildstr::BuildStr;
/// ///
/// #[derive(BuildStr)]
/// struct Person {
///     name: String,
///     surname: &'static str,    
///     initial: char,
///     age: u8,
///     is_human: bool,
/// }
/// ```
#[cfg(feature = "derive")]
#[proc_macro_derive(BuildStr)]
pub fn buildstr(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = input.ident;
    let generics = {
        let mut generics = input.generics;
        for param in &mut generics.params {
            if let syn::GenericParam::Type(ref mut type_param) = *param {
                type_param
                    .bounds
                    .push(syn::parse_quote!(buildstr::BuildStr));
            }
        }
        generics
    };
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let body = match input.data {
        syn::Data::Struct(ref s) => parse_struct(s, &name),
        syn::Data::Enum(e) => parse_enum(e, &name),
        syn::Data::Union(_) => panic!("Unions are not supported"),
    };

    quote! {
        #[allow(clippy::needless_borrow)]
        impl #impl_generics BuildStr for #name #ty_generics #where_clause {
            fn to_build_string(&self) -> String {
                #body
            }
        }
    }
    .into()
}

#[cfg(feature = "derive")]
fn parse_enum(e: syn::DataEnum, name: &syn::Ident) -> proc_macro2::TokenStream {
    let variants = e.variants.iter().map(|v| {
        let variant = &v.ident;
        match v.fields {
            syn::Fields::Named(ref fields) => {
                let fields = fields.named.iter().map(|field| {
                    &field.ident
                }).collect::<Vec<_>>();

                quote! {
                    #name::#variant { #(#fields),* } => {
                        let mut s = format!("{}::{}{{", stringify!(#name), stringify!(#variant));
                        #(
                            let f = format!("{}: {},", stringify!(#fields), #fields.to_build_string());
                            s.push_str(&f);
                        )*
                        s.push('}');
                        s
                    }
                }
            },
            syn::Fields::Unnamed(ref fields) => {
                let fields = (0..fields.unnamed.len()).map(|i| {
                    syn::parse_str::<syn::Ident>(&format!("_{i}")).unwrap()
                }).collect::<Vec<_>>();

                quote! {
                    #name::#variant( #(#fields),* ) => {
                        let mut s = format!("{}::{}(", stringify!(#name), stringify!(#variant));
                        #(
                            s.push_str(&#fields.to_build_string());
                            s.push(',');
                        )*
                        s.push(')');
                        s
                    }
                }
            }
            syn::Fields::Unit => quote! {
                #name::#variant => format!("{}::{}", stringify!(#name), stringify!(#variant)),
            },
        }
    });
    quote! {
        match self {
            #(#variants)*
        }
    }
}

#[cfg(feature = "derive")]
fn parse_struct(s: &syn::DataStruct, name: &syn::Ident) -> proc_macro2::TokenStream {
    match s.fields {
        syn::Fields::Named(ref fields) => {
            let fields = fields.named.iter().map(|field| {
                let name = &field.ident;
                quote_spanned! {field.span()=>
                    format!("{}: {},", stringify!(#name), (&self.#name).to_build_string())
                }
            });

            quote! {
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
                    format!("{},", (&self.#name).to_build_string())
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
                stringify!(#name).into()
            }
        }
    }
}

/// Creates a local implementation of the `BuildStr` trait, to allow implementing it on foreign types.<br>
/// This works because the [`BuildStr`] derive macro is unhygienic, allowing mixed trait implementations.
///
/// The macro will create implementations for all generic data structures based on the specified feature flags,<br>
/// meaning that you will not need to implement `Vec<T>` manually, only `T`.
///
/// # Examples
/// ```
/// use buildstr::{impl_buildstr, BuildStr};
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
        /// Trait for getting a string representation of the builder of a type.
        ///
        /// Supports all `std` types, arbitrary structs and enums.<br>
        /// Unions are *not* supported, you must implement `BuildStr` manually.
        ///
        /// Useful for macros that generate values at compile time, like parsers.
        ///
        /// If you want a pretty output, check the [`Pretty`](crate::Pretty) trait.
        ///
        /// To implement it for foreign types, use [`impl_buildstr!`](crate::impl_buildstr).
        ///
        /// # Examples
        /// ```
        /// use buildstr::BuildStr;
        ///         ///
        /// #[derive(BuildStr)]
        /// struct Person {
        ///     name: String,
        ///     age: u8,
        ///     balance: f64,
        /// }
        ///
        /// let person = Person {
        ///     name: "John".to_string(),
        ///     age: 30,
        ///     balance: 1000.
        /// };
        /// assert_eq!((&person).to_build_string(), "Person{name: ::std::string::String::from(\"John\"),age: 30u8,balance: 1000f64,}");
        /// ```
        pub trait BuildStr {
            /// Gets a string representation of the builder of a type.
            ///
            /// Useful for macros that generate values at compile time, like parsers.
            ///
            /// If you want a pretty output, check the [`Pretty`](crate::Pretty) trait.
            ///
            /// If you want to get the `TokenStream` directly, use [`to_build_tokens`](Self::to_build_tokens).
            /// # Examples
            /// ```
            /// use buildstr::BuildStr;
            ///             ///
            /// #[derive(BuildStr)]
            /// struct Person {
            ///     name: String,
            ///     age: u8,
            ///     balance: f64,
            /// }
            ///
            /// let person = Person {
            ///     name: "John".to_string(),
            ///     age: 30,
            ///     balance: 1000.
            /// };
            /// assert_eq!((&person).to_build_string(), "Person{name: ::std::string::String::from(\"John\"),age: 30u8,balance: 1000f64,}");
            /// ```
            fn to_build_string(&self) -> String;

            $to_build_tokens
        }
    }
    .to_owned();

    if cfg!(feature = "proc-macro") {
        let to = stringify! {
            /// Gets the TokenStream representation of the builder of a type.
            ///
            /// Useful for macros that generate values at compile time, like parsers.
            ///
            /// For more information see [`to_build_string`](Self::to_build_string).
            fn to_build_tokens(&self) -> buildstr::__private::TokenStream {
                buildstr::__private::__str_to_tokens(self.to_build_string())
            }
        };
        out = out.replace("$to_build_tokens", to);
    } else {
        out = out.replace("$to_build_tokens", "");
    }

    macro_rules! add_impls {
        ( $($feature:literal => [$($name:ident),*])* ) => {
            $(
                #[cfg(feature = $feature)] {
                    $(out.push_str($name());)*
                }
            )*
        }
    }

    add_impls! {
        "pretty" => [pretty]
        "prelude" => [
            option,
            result,
            r#box,
            rc,
            array,
            vec,
            tuple,
            reference
        ]
        "extra" => [
            borrow,
            cmp,
            collections,
            convert,
            time,
            cell,
            fmt,
            future,
            hash,
            marker,
            mem,
            net,
            num,
            ops,
            panic,
            pin,
            ffi
        ]
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
                buildstr::__private::__pretty((&self).to_build_string())
            }
        }
    }
}

impls! {

fn option() {
    impl<T: BuildStr> BuildStr for Option<T> {
        fn to_build_string(&self) -> String {
            match self {
                Some(s) => format!("::core::option::Some({})", s.to_build_string()),
                None => ::std::string::String::from("::core::option::None"),
            }
        }
    }
}

fn result() {
    impl<T, E> BuildStr for Result<T, E> where T: BuildStr, E: BuildStr {
        fn to_build_string(&self) -> String {
            match self {
                Ok(s) => format!("::core::result::Result::Ok({})", s.to_build_string()),
                Err(s) => format!("::core::result::Result::Err({})", s.to_build_string()),
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
    impl<T: BuildStr> BuildStr for ::std::rc::Rc<T> {
        fn to_build_string(&self) -> String {
            format!("Rc::new({})", self.as_ref().to_build_string())
        }
    }
    impl<T: BuildStr> BuildStr for ::std::rc::Weak<T> {
        fn to_build_string(&self) -> String {
            match self.upgrade() {
                Some(s) => format!(
                    "::std::rc::Rc::downgrade(&::std::rc::Rc::new({}))",
                    s.as_ref().to_build_string()
                ),
                None => ::std::string::String::from("::std::rc::Weak::new()"),
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
            format!("::std::vec::Vec::from_iter([{}])", buildstr::array_to_build_string!(self))
        }
    }
}

fn tuple() {
    impl<A: BuildStr> BuildStr for (A,) {
        fn to_build_string(&self) -> String {
            format!("({},)", (&self.0).to_build_string())
        }
    }
    impl<A, B> BuildStr for (A, B) where A: BuildStr, B: BuildStr {
        fn to_build_string(&self) -> String {
            format!(
                "({}, {})",
                (&self.0).to_build_string(),
                (&self.1).to_build_string()
            )
        }
    }
    impl<A, B, C> BuildStr for (A, B, C)
    where A: BuildStr, B: BuildStr, C: BuildStr {
        fn to_build_string(&self) -> String {
            format!(
                "({}, {}, {})",
                (&self.0).to_build_string(),
                (&self.1).to_build_string(),
                (&self.2).to_build_string()
            )
        }
    }
    impl<A, B, C, D> BuildStr for (A, B, C, D)
    where A: BuildStr, B: BuildStr, C: BuildStr, D: BuildStr {
        fn to_build_string(&self) -> String {
            format!(
                "({}, {}, {}, {})",
                (&self.0).to_build_string(),
                (&self.1).to_build_string(),
                (&self.2).to_build_string(),
                (&self.3).to_build_string()
            )
        }
    }
    impl<A, B, C, D, E> BuildStr for (A, B, C, D, E)
    where A: BuildStr, B: BuildStr, C: BuildStr, D: BuildStr, E: BuildStr {
        fn to_build_string(&self) -> String {
            format!(
                "({}, {}, {}, {}, {})",
                (&self.0).to_build_string(),
                (&self.1).to_build_string(),
                (&self.2).to_build_string(),
                (&self.3).to_build_string(),
                (&self.4).to_build_string()
            )
        }
    }
    impl<A, B, C, D, E, F> BuildStr for (A, B, C, D, E, F)
    where A: BuildStr, B: BuildStr, C: BuildStr, D: BuildStr, E: BuildStr, F: BuildStr {
        fn to_build_string(&self) -> String {
            format!(
                "({}, {}, {}, {}, {}, {})",
                (&self.0).to_build_string(),
                (&self.1).to_build_string(),
                (&self.2).to_build_string(),
                (&self.3).to_build_string(),
                (&self.4).to_build_string(),
                (&self.5).to_build_string()
            )
        }
    }
    impl<A, B, C, D, E, F, G> BuildStr for (A, B, C, D, E, F, G)
    where A: BuildStr, B: BuildStr, C: BuildStr, D: BuildStr, E: BuildStr, F: BuildStr, G: BuildStr {
        fn to_build_string(&self) -> String {
            format!(
                "({}, {}, {}, {}, {}, {}, {})",
                (&self.0).to_build_string(),
                (&self.1).to_build_string(),
                (&self.2).to_build_string(),
                (&self.3).to_build_string(),
                (&self.4).to_build_string(),
                (&self.5).to_build_string(),
                (&self.6).to_build_string()
            )
        }
    }
    impl<A, B, C, D, E, F, G, H> BuildStr for (A, B, C, D, E, F, G, H)
    where A: BuildStr, B: BuildStr, C: BuildStr, D: BuildStr, E: BuildStr, F: BuildStr, G: BuildStr, H: BuildStr {
        fn to_build_string(&self) -> String {
            format!(
                "({}, {}, {}, {}, {}, {}, {}, {})",
                (&self.0).to_build_string(),
                (&self.1).to_build_string(),
                (&self.2).to_build_string(),
                (&self.3).to_build_string(),
                (&self.4).to_build_string(),
                (&self.5).to_build_string(),
                (&self.6).to_build_string(),
                (&self.7).to_build_string(),
            )
        }
    }
    impl<A, B, C, D, E, F, G, H, I> BuildStr for (A, B, C, D, E, F, G, H, I)
    where A: BuildStr, B: BuildStr, C: BuildStr, D: BuildStr, E: BuildStr, F: BuildStr, G: BuildStr, H: BuildStr, I: BuildStr {
        fn to_build_string(&self) -> String {
            format!(
                "({}, {}, {}, {}, {}, {}, {}, {}, {})",
                (&self.0).to_build_string(),
                (&self.1).to_build_string(),
                (&self.2).to_build_string(),
                (&self.3).to_build_string(),
                (&self.4).to_build_string(),
                (&self.5).to_build_string(),
                (&self.6).to_build_string(),
                (&self.7).to_build_string(),
                (&self.8).to_build_string(),
            )
        }
    }
    impl<A, B, C, D, E, F, G, H, I, J> BuildStr for (A, B, C, D, E, F, G, H, I, J)
    where A: BuildStr, B: BuildStr, C: BuildStr, D: BuildStr, E: BuildStr, F: BuildStr, G: BuildStr, H: BuildStr, I: BuildStr, J: BuildStr {
        fn to_build_string(&self) -> String {
            format!(
                "({}, {}, {}, {}, {}, {}, {}, {}, {}, {})",
                (&self.0).to_build_string(),
                (&self.1).to_build_string(),
                (&self.2).to_build_string(),
                (&self.3).to_build_string(),
                (&self.4).to_build_string(),
                (&self.5).to_build_string(),
                (&self.6).to_build_string(),
                (&self.7).to_build_string(),
                (&self.8).to_build_string(),
                (&self.9).to_build_string(),
            )
        }
    }
    impl<A, B, C, D, E, F, G, H, I, J, K> BuildStr for (A, B, C, D, E, F, G, H, I, J, K)
    where A: BuildStr, B: BuildStr, C: BuildStr, D: BuildStr, E: BuildStr, F: BuildStr, G: BuildStr, H: BuildStr, I: BuildStr, J: BuildStr, K: BuildStr {
        fn to_build_string(&self) -> String {
            format!(
                "({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})",
                (&self.0).to_build_string(),
                (&self.1).to_build_string(),
                (&self.2).to_build_string(),
                (&self.3).to_build_string(),
                (&self.4).to_build_string(),
                (&self.5).to_build_string(),
                (&self.6).to_build_string(),
                (&self.7).to_build_string(),
                (&self.8).to_build_string(),
                (&self.9).to_build_string(),
                (&self.10).to_build_string(),
            )
        }
    }
    impl<A, B, C, D, E, F, G, H, I, J, K, L> BuildStr for (A, B, C, D, E, F, G, H, I, J, K, L)
    where A: BuildStr, B: BuildStr, C: BuildStr, D: BuildStr, E: BuildStr, F: BuildStr, G: BuildStr, H: BuildStr, I: BuildStr, J: BuildStr, K: BuildStr, L: BuildStr {
        fn to_build_string(&self) -> String {
            format!(
                "({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})",
                (&self.0).to_build_string(),
                (&self.1).to_build_string(),
                (&self.2).to_build_string(),
                (&self.3).to_build_string(),
                (&self.4).to_build_string(),
                (&self.5).to_build_string(),
                (&self.6).to_build_string(),
                (&self.7).to_build_string(),
                (&self.8).to_build_string(),
                (&self.9).to_build_string(),
                (&self.10).to_build_string(),
                (&self.11).to_build_string(),
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

fn borrow() {
    impl<'a, T: ::std::borrow::ToOwned + ?Sized> BuildStr for ::std::borrow::Cow<'a, T> where <T as ToOwned>::Owned: BuildStr, &'a T: BuildStr {
        fn to_build_string(&self) -> String {
            let ty = ::std::any::type_name::<T>();
            match self {
                ::std::borrow::Cow::Borrowed(b) => format!("::std::borrow::Cow::Borrowed::<{ty}>({})", (*b).to_build_string()),
                ::std::borrow::Cow::Owned(o) => format!("::std::borrow::Cow::Owned::<{ty}>({})", o.to_build_string()),
            }
        }
    }
}

fn cell() {
    impl <T: BuildStr> BuildStr for ::core::cell::Cell<T> {
        fn to_build_string(&self) -> String {
            let v = self.as_ptr();
            // SAFETY: The pointer must be valid, as the cell is always initialized
            if let Some(v) = unsafe { v.as_ref() } {
                format!("::core::cell::Cell::new({})", (&v).to_build_string())
            } else {
                panic!("Invalid pointer in ::core::cell::Cell, can't convert to BuildStr");
            }
        }
    }
    // TODO: Needs testing
    impl <T: BuildStr> BuildStr for ::core::cell::OnceCell<T> {
        fn to_build_string(&self) -> String {
            if let Some(v) = self.get() {
                format!("{{
                    let cell = ::core::cell::OnceCell::new(); 
                    let _ = cell.set({});
                    cell
                }}", v.to_build_string())
            } else {
                "::core::cell::OnceCell::new()".into()
            }
        }
    }
    impl <T: BuildStr> BuildStr for ::core::cell::RefCell<T> {
        fn to_build_string(&self) -> String {
            format!("::core::cell::RefCell::new({})", self.borrow().to_build_string())
        }
    }
    // TODO: Needs testing
    impl <T: BuildStr> BuildStr for ::core::cell::UnsafeCell<T> {
        fn to_build_string(&self) -> String {
            let v = self.get();
            // SAFETY: The pointer must be valid, as the cell is always initialized
            if let Some(v) = unsafe { v.as_ref() } {
                format!("::core::cell::UnsafeCell::new({})", v.to_build_string())
            } else {
                panic!("Invalid pointer in ::core::cell::UnsafeCell, can't convert to BuildStr");
            }
        }
    }
}

fn collections() {
    impl<K, V> BuildStr for ::std::collections::BTreeMap<K, V> where K: BuildStr + ::core::cmp::Ord, V: BuildStr {
        fn to_build_string(&self) -> String {
            format!("::std::collections::BTreeMap::from_iter([{}])", buildstr::map_to_build_string!(self))
        }
    }
    impl<T: BuildStr> BuildStr for ::std::collections::BTreeSet<T> {
        fn to_build_string(&self) -> String {
            format!("::std::collections::BTreeSet::from_iter([{}])", buildstr::array_to_build_string!(self))
        }
    }
    impl<T: BuildStr> BuildStr for ::std::collections::BinaryHeap<T> {
        fn to_build_string(&self) -> String {
            format!("::std::collections::BinaryHeap::from_iter([{}])", buildstr::array_to_build_string!(self))
        }
    }
    impl<K, V, S> BuildStr for ::std::collections::HashMap<K, V, S> where K: BuildStr, V: BuildStr {
        fn to_build_string(&self) -> String {
            format!("::std::collections::HashMap::from_iter([{}])", buildstr::map_to_build_string!(self))
        }
    }
    impl<T: BuildStr, S> BuildStr for ::std::collections::HashSet<T, S> {
        fn to_build_string(&self) -> String {
            format!("::std::collections::HashSet::from_iter([{}])", buildstr::array_to_build_string!(self))
        }
    }
    impl<T: BuildStr> BuildStr for ::std::collections::LinkedList<T> {
        fn to_build_string(&self) -> String {
            format!("::std::collections::LinkedList::from_iter([{}])", buildstr::array_to_build_string!(self))
        }
    }
    impl<T: BuildStr> BuildStr for ::std::collections::VecDeque<T> {
        fn to_build_string(&self) -> String {
            format!("::std::collections::VecDeque::from_iter([{}])", buildstr::array_to_build_string!(self))
        }
    }
}

fn cmp() {
    impl<T: BuildStr> BuildStr for ::core::cmp::Reverse<T> {
        fn to_build_string(&self) -> String {
            format!("::core::cmp::Reverse({})", self.0.to_build_string())
        }
    }
}

fn convert() {
    impl<T: ?Sized> BuildStr for &dyn std::convert::AsRef<T> where for<'a> &'a T: BuildStr {
        fn to_build_string(&self) -> String {
            let r = self.as_ref().to_build_string();
            let ty = std::any::type_name::<T>();
            format!("&{r} as &dyn ::std::convert::AsRef<{ty}>")
        }
    }
}

fn ffi() {

}

fn fmt() {

}

fn future() {
    impl<T> BuildStr for ::core::future::Pending<T> {
        fn to_build_string(&self) -> String {
            format!("::core::future::pending::<{}>()", ::core::any::type_name::<T>())
        }
    }
}

fn hash() {
    impl<H> BuildStr for ::core::hash::BuildHasherDefault<H> {
        fn to_build_string(&self) -> String {
            format!("{}::default()", ::std::any::type_name::<Self>())
        }
    }
}

fn marker() {
    impl<T> BuildStr for ::core::marker::PhantomData<T> {
        fn to_build_string(&self) -> String {
            format!("::core::marker::PhantomData::<{}>", ::std::any::type_name::<T>())
        }
    }
}

fn mem() {
    impl<T: BuildStr> BuildStr for ::core::mem::ManuallyDrop<T> {
        fn to_build_string(&self) -> String {
            format!("::core::mem::ManuallyDrop::new({})", (**self).to_build_string())
        }
    }
}

fn net() {

}

fn num() {
    impl<T: BuildStr> BuildStr for ::core::num::Wrapping<T> {
        fn to_build_string(&self) -> String {
            format!("::core::num::Wrapping({})", self.0.to_build_string())
        }
    }
}

fn ops() {
    impl<T: BuildStr> BuildStr for ::core::ops::Bound<T> {
        fn to_build_string(&self) -> String {
            match self {
                ::core::ops::Bound::Included(i) => format!("::core::ops::Bound::Included({})", i.to_build_string()),
                ::core::ops::Bound::Excluded(e) => format!("::core::ops::Bound::Excluded({})", e.to_build_string()),
                ::core::ops::Bound::Unbounded => "::core::ops::Bound::Unbounded".into(),
            }
        }
    }
    impl<B, C> BuildStr for ::core::ops::ControlFlow<B, C> where B: BuildStr, C: BuildStr {
        fn to_build_string(&self) -> String {
            match self {
                ::core::ops::ControlFlow::Continue(c) => format!("::core::ops::ControlFlow::Continue({})", c.to_build_string()),
                ::core::ops::ControlFlow::Break(b) => format!("::core::ops::ControlFlow::Break({})", b.to_build_string()),
            }
        }
    }
    impl<Idx: BuildStr> BuildStr for ::core::ops::Range<Idx> {
        fn to_build_string(&self) -> String {
            let start = self.start.to_build_string();
            let end = self.end.to_build_string();
            format!("::core::ops::Range{{start:{start},end:{end}}}")
        }
    }
    impl<Idx: BuildStr> BuildStr for ::core::ops::RangeFrom<Idx> {
        fn to_build_string(&self) -> String {
            let start = self.start.to_build_string();
            format!("::core::ops::RangeFrom{{start:{start}}}")
        }
    }
    impl<Idx: BuildStr> BuildStr for ::core::ops::RangeInclusive<Idx> {
        fn to_build_string(&self) -> String {
            let start = self.start().to_build_string();
            let end = self.end().to_build_string();
            format!("::core::ops::RangeInclusive{{start:{start},end:{end}}}")
        }
    }
    impl<Idx: BuildStr> BuildStr for ::core::ops::RangeTo<Idx> {
        fn to_build_string(&self) -> String {
            let end = self.end.to_build_string();
            format!("::core::ops::RangeTo {{end:{end}}}")
        }
    }
    impl<Idx: BuildStr> BuildStr for ::core::ops::RangeToInclusive<Idx> {
        fn to_build_string(&self) -> String {
            let end = self.end.to_build_string();
            format!("::core::ops::RangeToInclusive{{end:{end}}}")
        }
    }
}

fn panic() {
    impl<T: BuildStr> BuildStr for ::core::panic::AssertUnwindSafe<T> {
        fn to_build_string(&self) -> String {
            format!("::core::panic::AssertUnwindSafe({})", self.0.to_build_string())
        }
    }
}

fn pin() {
    impl<T: Unpin + BuildStr + ::core::ops::Deref> BuildStr for ::std::pin::Pin<T> {
        fn to_build_string(&self) -> String {
            // SAFETY: ::std::pin::Pin<T> is repr(transparent), so we can safely downcast it
            let ptr: &T = unsafe { &*(self as *const std::pin::Pin<T>).cast() };
            format!("::std::pin::Pin::new({})", ptr.to_build_string())
        }
    }
}

fn time() {

}

}

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, parse_quote, spanned::Spanned, DeriveInput, Fields};

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

#[proc_macro_derive(BuildStr)]
pub fn buildstr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let body = match input.data {
        syn::Data::Struct(ref s) => match s.fields {
            Fields::Named(ref fields) => {
                let fields = fields.named.iter().map(|field| {
                    let name = &field.ident;
                    quote_spanned! {field.span()=>
                        format!("{}:{},", stringify!(#name), self.#name.to_build_string())
                    }
                });

                quote! {
                    let mut s = format!("{}{{", stringify!(#name));
                    #(s.push_str(&#fields);)*
                    s.push('}');
                    s
                }
            }
            Fields::Unnamed(ref fields) => {
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
            Fields::Unit => {
                quote! {
                    impl BuildStr for #name {
                        fn to_build_string(&self) -> String {
                            format!("{}", #name)
                        }
                    }
                }
            }
        },
        syn::Data::Enum(e) => todo!(),
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

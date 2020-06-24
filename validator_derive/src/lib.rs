extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(ValidatorBase)]
pub fn validator_base_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_init(&ast)
}

fn impl_init(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl ValidatorBase for #name {
            fn init() -> String {
                format!("Hello! I am {}!", stringify!(#name))
            }
        }
    };
    gen.into()
}

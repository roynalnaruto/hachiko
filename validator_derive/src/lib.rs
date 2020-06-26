extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse, parse_macro_input, punctuated::Punctuated, token::Comma, Field, Fields, FieldsNamed,
    ItemStruct,
};

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

#[proc_macro_attribute]
pub fn add_base_state(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(item as ItemStruct);
    let field_b: FieldsNamed = syn::parse_str("{ last_block: U64 }").expect("should not fail");
    let field_b: Punctuated<Field, Comma> = field_b.named;
    let field_b_tokens = field_b.to_token_stream();

    let new_fields = match item.fields.clone() {
        Fields::Named(named_fields) => {
            let mut tokens = named_fields.named.to_token_stream();
            tokens.extend(field_b_tokens);
            let braced = quote! {{ #tokens }};
            let named_fields =
                parse::<FieldsNamed>(TokenStream::from(braced)).expect("should not fail");

            Fields::Named(named_fields)
        }
        _ => panic!("only named fields allowed"),
    };

    item.fields = new_fields;

    let gen = quote! { #item };

    gen.into()
}

#[proc_macro_attribute]
pub fn add_base_state_transition(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(item as ItemStruct);
    let field_b: FieldsNamed =
        syn::parse_str("{ tx_receipt: TransactionReceipt }").expect("should not fail");
    let field_b: Punctuated<Field, Comma> = field_b.named;
    let field_b_tokens = field_b.to_token_stream();

    let new_fields = match item.fields.clone() {
        Fields::Named(named_fields) => {
            let mut tokens = named_fields.named.to_token_stream();
            tokens.extend(field_b_tokens);
            let braced = quote! {{ #tokens }};
            let named_fields =
                parse::<FieldsNamed>(TokenStream::from(braced)).expect("should not fail");

            Fields::Named(named_fields)
        }
        _ => panic!("only named fields allowed"),
    };

    item.fields = new_fields;

    let gen = quote! { #item };

    gen.into()
}

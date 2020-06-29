extern crate proc_macro;

use inflector::cases::snakecase::to_snake_case;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::{
    parse, parse_macro_input, punctuated::Punctuated, token::Comma, Data, DataStruct, Field,
    Fields, FieldsNamed, ItemStruct,
};

#[proc_macro_derive(BaseState)]
pub fn base_state_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_base_state(&ast)
}

fn impl_base_state(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let block_ident = Some(Ident::new("last_block", Span::call_site()));
    let fields = match &ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };
    let field_name: Vec<&Option<Ident>> = fields
        .into_iter()
        .filter(|field| !field.ident.eq(&block_ident))
        .map(|field| &field.ident)
        .collect();

    let gen = quote! {
        impl State for #name {
            fn get_state(&self) -> Self {
                self.clone()
            }

            fn get_last_block(&self) -> Option<U64> {
                self.last_block
            }
        }

        impl PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                #(
                    if self.#field_name != other.#field_name {
                        return false;
                    }
                )*

                true
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(BaseStateTransition)]
pub fn base_state_transition_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_base_state_transition(&ast)
}

fn impl_base_state_transition(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl StateTransition for #name {
            fn get_receipt(&self) -> TransactionReceipt {
                self.tx_receipt.clone()
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(Configurable)]
pub fn configurable_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_fetch_config(&ast)
}

fn impl_fetch_config(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let config_filename = to_snake_case(&name.to_string());
    let config_path = format!("config/default/{}", config_filename);

    let gen = quote! {
        impl Configurable for #name {
            fn fetch_config() -> ValidatorConfig {
                let mut s = Config::new();
                s.merge(File::with_name(#config_path)).expect("[load config] should not fail");
                let c: FetchConfig = s.try_into().expect("[parse config] should not fail");
                let pk = PrivateKey::from_str(c.private_key.as_str()).expect("[parse pk] should not fail");
                let wallet: Wallet = pk.into();
                let addr = Address::from_str(c.address.as_str()).expect("[parse addr] should not fail");

                ValidatorConfig::new(&wallet, &addr, c.url.as_str())
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(ValidatorBase)]
pub fn validator_base_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_init(&ast)
}

fn impl_init(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let state_ident = name.to_string().replace("Validator", "State");
    let state_ident = Ident::new(state_ident.as_str(), Span::call_site());
    let state_transition_ident = name.to_string().replace("Validator", "StateTransition");
    let state_transition_ident = Ident::new(state_transition_ident.as_str(), Span::call_site());

    let gen = quote! {
        impl ValidatorBase for #name {
            fn init() -> Self {
                let config = Self::fetch_config();
                let provider = Provider::<Http>::try_from(config.url)
                    .expect("should not fail")
                    .interval(Duration::from_millis(10u64));
                let client = config.wallet.connect(provider);
                let client = Arc::new(client);
                let contract: SimpleStorage<Http, Wallet> =
                    SimpleStorage::new(config.address, client.clone());

                SimpleStorageValidator {
                    contract: contract,
                    state: #state_ident::default(),
                    state_transition: #state_transition_ident::default(),
                }
            }

            fn init_with(config: ValidatorConfig) -> Self {
                let provider = Provider::<Http>::try_from(config.url)
                    .expect("should not fail")
                    .interval(Duration::from_millis(10u64));
                let client = config.wallet.connect(provider);
                let client = Arc::new(client);
                let contract: SimpleStorage<Http, Wallet> =
                    SimpleStorage::new(config.address.clone(), client.clone());

                SimpleStorageValidator {
                    contract: contract,
                    state: #state_ident::default(),
                    state_transition: #state_transition_ident::default(),
                }
            }
        }
    };

    gen.into()
}

#[proc_macro_attribute]
pub fn add_base_state(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(item as ItemStruct);
    let field_b: FieldsNamed =
        syn::parse_str("{ #[builder(default = \"None\")]last_block: Option<U64> }")
            .expect("should not fail");
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
pub fn add_base_state_transition(event_type: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(item as ItemStruct);
    let add_fields = format!(
        "{{
        tx_receipt: TransactionReceipt, \
        #[builder(default = \"None\")] \
        last_events: Option<Vec<{}>>, \
    }}",
        event_type.to_string()
    );
    let fields: FieldsNamed = syn::parse_str(&add_fields).expect("should not fail");
    let fields: Punctuated<Field, Comma> = fields.named;
    let fields_tokens = fields.to_token_stream();

    let new_fields = match item.fields.clone() {
        Fields::Named(named_fields) => {
            let mut tokens = named_fields.named.to_token_stream();
            tokens.extend(fields_tokens);
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

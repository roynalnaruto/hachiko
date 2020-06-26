extern crate proc_macro;

use inflector::cases::snakecase::to_snake_case;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::{
    Field, Fields, FieldsNamed, ItemStruct,
    parse, parse_macro_input,
    punctuated::Punctuated,
    token::Comma,
};

#[proc_macro_derive(ValidatorBase)]
pub fn validator_base_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_init(&ast)
}

fn impl_init(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let config_name = format!("{}Config", name);
    let config_filename = to_snake_case(&name.to_string());
    let config_path = format!("config/default/{}", config_filename);
    let config_ident = Ident::new(config_name.as_str(), Span::call_site());

    let gen = quote! {
        #[derive(Debug, Deserialize)]
        pub struct #config_ident {
            private_key: String,
            address: String,
            url: String,
        }

        impl Configurable for #config_ident {
            fn fetch_config() -> ValidatorConfig {
                let mut s = Config::new();
                s.merge(File::with_name(#config_path)).expect("[load config] should not fail");
                let c: #config_ident = s.try_into().expect("[parse config] should not fail");
                let pk = PrivateKey::from_str(c.private_key.as_str()).expect("[parse pk] should not fail");
                let wallet: Wallet = pk.into();
                let addr = Address::from_str(c.address.as_str()).expect("[parse addr] should not fail");

                ValidatorConfig::new(&wallet, &addr, c.url.as_str())
            }
        }

        impl ValidatorBase for #name {
            fn init() -> Self {
                let config = #config_ident::fetch_config();
                let provider = Provider::<Http>::try_from(config.url)
                    .expect("should not fail")
                    .interval(Duration::from_millis(10u64));
                let client = config.wallet.connect(provider);
                let client = Arc::new(client);
                let address = config.address.clone();
                let contract: SimpleStorage<Http, Wallet> =
                    SimpleStorage::new(address, client.clone());

                SimpleStorageValidator { contract }
            }

            fn init_with(config: ValidatorConfig) -> Self {
                let provider = Provider::<Http>::try_from(config.url)
                    .expect("should not fail")
                    .interval(Duration::from_millis(10u64));
                let client = config.wallet.connect(provider);
                let client = Arc::new(client);
                let address = config.address.clone();
                let contract: SimpleStorage<Http, Wallet> =
                    SimpleStorage::new(address, client.clone());

                SimpleStorageValidator { contract }
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

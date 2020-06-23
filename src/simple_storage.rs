pub use simplestorage_mod::*;
mod simplestorage_mod {
    #![allow(dead_code)]
    #![allow(unused_imports)]
    use ethers::{
        contract::{
            builders::{ContractCall, Event},
            Contract, Lazy,
        },
        core::{
            abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
            types::*,
        },
        providers::JsonRpcClient,
        signers::{Client, Signer},
    };
    #[doc = "SimpleStorage was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static SIMPLESTORAGE_ABI: Lazy<Abi> = Lazy::new(|| {
        serde_json :: from_str ( "[{\"inputs\":[{\"internalType\":\"string\",\"name\":\"value\",\"type\":\"string\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"author\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"oldAuthor\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"string\",\"name\":\"oldValue\",\"type\":\"string\"},{\"indexed\":false,\"internalType\":\"string\",\"name\":\"newValue\",\"type\":\"string\"}],\"name\":\"ValueChanged\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"getValue\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"lastSender\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"value\",\"type\":\"string\"}],\"name\":\"setValue\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]" ) . expect ( "invalid abi" )
    });
    #[derive(Clone)]
    pub struct SimpleStorage<P, S>(Contract<P, S>);
    impl<P, S> std::ops::Deref for SimpleStorage<P, S> {
        type Target = Contract<P, S>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<P: JsonRpcClient, S: Signer> std::fmt::Debug for SimpleStorage<P, S> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(SimpleStorage))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, P: JsonRpcClient, S: Signer> SimpleStorage<P, S> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<Address>, C: Into<Arc<Client<P, S>>>>(address: T, client: C) -> Self {
            let contract = Contract::new(address.into(), SIMPLESTORAGE_ABI.clone(), client.into());
            Self(contract)
        }
        #[doc = "Calls the contract's `setValue` (0x93a09352) function"]
        pub fn set_value(&self, value: String) -> ContractCall<P, S, H256> {
            self.0
                .method_hash([147, 160, 147, 82], (value,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getValue` (0x20965255) function"]
        pub fn get_value(&self) -> ContractCall<P, S, String> {
            self.0
                .method_hash([32, 150, 82, 85], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lastSender` (0x256fec88) function"]
        pub fn last_sender(&self) -> ContractCall<P, S, Address> {
            self.0
                .method_hash([37, 111, 236, 136], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `ValueChanged` event"]
        pub fn value_changed_filter(&self) -> Event<P, ValueChangedFilter> {
            self.0
                .event("ValueChanged")
                .expect("event not found (this should never happen)")
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct ValueChangedFilter {
        pub author: Address,
        pub old_author: Address,
        pub old_value: String,
        pub new_value: String,
    }
    impl ValueChangedFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                153, 155, 109, 70, 76, 78, 51, 131, 195, 65, 189, 211, 162, 43, 2, 221, 168, 167,
                225, 214, 156, 6, 157, 37, 46, 53, 203, 46, 226, 244, 163, 195,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`ValueChanged(address,address,string,string)`"]
        pub const fn abi_signature() -> &'static str {
            "ValueChanged(address,address,string,string)"
        }
    }
    impl Detokenize for ValueChangedFilter {
        fn from_tokens(tokens: Vec<Token>) -> Result<Self, InvalidOutputType> {
            if tokens.len() != 4 {
                return Err(InvalidOutputType(format!(
                    "Expected {} tokens, got {}: {:?}",
                    4,
                    tokens.len(),
                    tokens
                )));
            }
            #[allow(unused_mut)]
            let mut tokens = tokens.into_iter();
            let author = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let old_author =
                Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let old_value =
                Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let new_value =
                Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(ValueChangedFilter {
                author,
                old_author,
                old_value,
                new_value,
            })
        }
    }
}

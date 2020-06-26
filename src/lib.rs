extern crate config;
extern crate serde;

use async_trait::async_trait;
use ethers::{
    contract::ContractError,
    core::types::{Address, TransactionReceipt},
    signers::Wallet,
};

mod simple_storage;

mod simple_storage_validator;

mod validate;

#[derive(Debug)]
pub struct ValidatorConfig {
    pub wallet: Wallet,
    pub address: Address,
    pub url: String,
}

impl ValidatorConfig {
    fn new(wallet: &Wallet, addr: &Address, url: &str) -> Self {
        ValidatorConfig {
            wallet: wallet.clone(),
            address: addr.clone(),
            url: url.to_string(),
        }
    }
}

pub trait Configurable {
    fn fetch_config() -> ValidatorConfig;
}

pub trait State: Clone + std::fmt::Debug + PartialEq + Sized {
    fn get_state(&self) -> Self;
}

pub trait StateTransition: Clone + std::fmt::Debug + PartialEq + Sized {
    fn get_receipt(&self) -> TransactionReceipt;
}

#[async_trait]
pub trait Validator<S, T>: Sized
where
    S: State,
    T: StateTransition,
{
    async fn before_state(&self) -> Result<S, ContractError>;

    async fn state_transition(&self, state: S) -> Result<(T, S), ContractError>;

    async fn after_state(&self) -> Result<S, ContractError>;
}

pub trait ValidatorBase {
    fn init() -> Self;

    fn init_with(config: ValidatorConfig) -> Self;
}

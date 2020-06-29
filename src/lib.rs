extern crate config;
extern crate serde;

#[macro_use]
extern crate derive_builder;

use async_trait::async_trait;
use ethers::{
    contract::ContractError,
    core::types::{Address, TransactionReceipt, U64},
    signers::Wallet,
};
use serde::Deserialize;

mod simple_storage;

mod simple_storage_validator;

mod validate;

#[derive(Debug, Deserialize)]
pub struct FetchConfig {
    private_key: String,
    address: String,
    url: String,
}

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

    fn get_last_block(&self) -> Option<U64>;
}

pub trait StateTransition: Clone + std::fmt::Debug + PartialEq + Sized {
    fn get_receipt(&self) -> TransactionReceipt;
}

#[async_trait]
pub trait Validator<S, T, E>: Sized
where
    S: State,
    T: StateTransition,
    E: std::fmt::Debug + PartialEq,
{
    fn get_state(&self) -> S;

    fn get_state_transition(&self) -> T;

    async fn fetch_state(&self) -> Result<S, ContractError>;

    async fn sync_state(&mut self) -> Result<S, ContractError>;

    async fn sync_events(&mut self, block: U64) -> Result<Vec<E>, ContractError>;

    async fn state_transition(&mut self, initial_state: S) -> Result<(S, Vec<E>), ContractError>;
}

pub trait ValidatorBase {
    fn init() -> Self;

    fn init_with(config: ValidatorConfig) -> Self;
}

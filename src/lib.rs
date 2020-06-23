use async_trait::async_trait;
use ethers::{
    contract::ContractError,
    core::types::{Address, PrivateKey, TransactionReceipt},
};

mod simple_storage;

mod simple_storage_validator;
pub use simple_storage_validator::SimpleStorageValidator;

mod validate;

pub struct ValidatorConfig {
    pub private_key: PrivateKey,
    pub address: Address,
    pub url: String,
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
    fn say_hi(&self);

    async fn before_state(&self) -> Result<S, ContractError>;

    async fn state_transition(&self) -> Result<T, ContractError>;

    async fn after_state(&self) -> Result<S, ContractError>;
}

use async_trait::async_trait;
use ethers::{
    contract::ContractError,
    core::types::{Address, TransactionReceipt},
    providers::{Http, Provider},
    signers::Wallet,
};

use std::{convert::TryFrom, sync::Arc};

use crate::{simple_storage::SimpleStorage, State, StateTransition, Validator, ValidatorConfig};

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct SimpleStorageState {
    value: String,
    last_sender: Address,
}

impl State for SimpleStorageState {
    fn get_state(&self) -> Self {
        self.clone()
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct SimpleStorageStateTransition {
    tx_receipt: TransactionReceipt,
}

impl StateTransition for SimpleStorageStateTransition {
    fn get_receipt(&self) -> TransactionReceipt {
        self.tx_receipt.clone()
    }
}

pub struct SimpleStorageValidator {
    contract: SimpleStorage<Http, Wallet>,
}

#[allow(dead_code)]
impl SimpleStorageValidator {
    pub fn new(config: ValidatorConfig) -> Self {
        let wallet: Wallet = config.private_key.clone().into();
        let provider = Provider::<Http>::try_from(config.url).expect("this should not fail");
        let client = wallet.connect(provider);
        let client = Arc::new(client);
        let contract: SimpleStorage<Http, Wallet> =
            SimpleStorage::new(config.address, client.clone());

        SimpleStorageValidator { contract }
    }
}

#[async_trait]
impl Validator<SimpleStorageState, SimpleStorageStateTransition> for SimpleStorageValidator {
    async fn before_state(&self) -> Result<SimpleStorageState, ContractError> {
        let value = self.contract.get_value().call().await?;
        let last_sender = self.contract.last_sender().call().await?;

        Ok(SimpleStorageState { value, last_sender })
    }

    async fn state_transition(&self) -> Result<SimpleStorageStateTransition, ContractError> {
        let tx_hash = self.contract.set_value("hi".to_owned()).send().await?;
        let tx_receipt = self.contract.pending_transaction(tx_hash).await?;

        Ok(SimpleStorageStateTransition { tx_receipt })
    }

    async fn after_state(&self) -> Result<SimpleStorageState, ContractError> {
        let value = self.contract.get_value().call().await?;
        let last_sender = self.contract.last_sender().call().await?;

        Ok(SimpleStorageState { value, last_sender })
    }
}

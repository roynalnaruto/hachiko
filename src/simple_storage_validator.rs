use async_trait::async_trait;
use ethers::{
    contract::ContractError,
    core::types::{Address, TransactionReceipt, U64},
    providers::{Http, Provider},
    signers::Wallet,
};
use validator_derive::{add_base_state, add_base_state_transition, ValidatorBase};

use std::{convert::TryFrom, sync::Arc};

use crate::{
    simple_storage::SimpleStorage, State, StateTransition, Validator, ValidatorBase,
    ValidatorConfig,
};

#[add_base_state]
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

#[add_base_state_transition]
#[derive(Clone, Debug, PartialEq)]
pub struct SimpleStorageStateTransition {}

impl StateTransition for SimpleStorageStateTransition {
    fn get_receipt(&self) -> TransactionReceipt {
        self.tx_receipt.clone()
    }
}

#[derive(ValidatorBase)]
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
        let last_block = self.contract.client().get_block_number().await?;

        Ok(SimpleStorageState {
            value,
            last_sender,
            last_block,
        })
    }

    async fn state_transition(
        &self,
        _state: SimpleStorageState,
    ) -> Result<(SimpleStorageStateTransition, SimpleStorageState), ContractError> {
        let tx_hash = self.contract.set_value("hi".to_owned()).send().await?;
        let tx_receipt = self.contract.pending_transaction(tx_hash).await?;

        Ok((
            SimpleStorageStateTransition { tx_receipt },
            SimpleStorageState {
                value: "hi".to_string(),
                last_sender: self.contract.client().address(),
                // TODO: remove by adding builder and making
                // last_block as Option<last_block>
                last_block: U64::default(),
            },
        ))
    }

    async fn after_state(&self) -> Result<SimpleStorageState, ContractError> {
        let value = self.contract.get_value().call().await?;
        let last_sender = self.contract.last_sender().call().await?;

        Ok(SimpleStorageState {
            value: value,
            last_sender: last_sender,
            // TODO: remove by adding builder and making
            // last_block as Option<last_block>
            last_block: U64::default(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_init() {
        assert_eq!(
            SimpleStorageValidator::init(),
            "Hello! I am SimpleStorageValidator!",
        );
    }
}

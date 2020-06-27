use async_trait::async_trait;
use config::{Config, File};
use ethers::{
    contract::ContractError,
    core::types::{Address, PrivateKey, TransactionReceipt, U64},
    providers::{Http, Provider},
    signers::Wallet,
};
use validator_derive::{
    add_base_state, add_base_state_transition, BaseState, BaseStateTransition, Configurable,
    ValidatorBase,
};

use std::{convert::TryFrom, str::FromStr, sync::Arc, time::Duration};

use crate::{
    simple_storage::SimpleStorage, Configurable, FetchConfig, State, StateTransition, Validator,
    ValidatorBase, ValidatorConfig,
};

#[add_base_state]
#[derive(BaseState, Clone, Debug, Default, Builder)]
pub struct SimpleStorageState {
    value: String,
    last_sender: Address,
}

#[add_base_state_transition]
#[derive(BaseStateTransition, Clone, Debug, Default, Builder, PartialEq)]
pub struct SimpleStorageStateTransition {}

#[derive(Configurable, ValidatorBase, Debug)]
pub struct SimpleStorageValidator {
    contract: SimpleStorage<Http, Wallet>,
    state: SimpleStorageState,
    state_transition: SimpleStorageStateTransition,
}

#[async_trait]
impl Validator<SimpleStorageState, SimpleStorageStateTransition> for SimpleStorageValidator {
    fn get_state(&self) -> SimpleStorageState {
        self.state.clone()
    }

    fn get_state_transition(&self) -> SimpleStorageStateTransition {
        self.state_transition.clone()
    }

    async fn fetch_state(&self) -> Result<SimpleStorageState, ContractError> {
        // 1. Fetch the most recent state from the blockchain
        let value = self.contract.get_value().call().await?;
        let last_sender = self.contract.last_sender().call().await?;
        let last_block = self.contract.client().get_block_number().await?;

        // 2. Build the state with the above values
        let state = SimpleStorageStateBuilder::default()
            .value(value)
            .last_sender(last_sender)
            .last_block(Some(last_block))
            .build()
            .unwrap();

        Ok(state)
    }

    async fn sync_state(&mut self) -> Result<SimpleStorageState, ContractError> {
        // 1. Get the Validator's most recent state
        let state = self.fetch_state().await?;

        // 2. Update the Validator's state
        self.state = state.clone();

        Ok(state)
    }

    async fn state_transition(
        &mut self,
        _initial_state: SimpleStorageState,
    ) -> Result<SimpleStorageState, ContractError> {
        // 1. Broadcast a transaction to execute state transition
        let tx_hash = self.contract.set_value("hi".to_owned()).send().await?;

        // 2. Get receipt for the transaction
        let tx_receipt = self.contract.pending_transaction(tx_hash).await?;

        // 3. Build the state transition struct
        let state_transition = SimpleStorageStateTransitionBuilder::default()
            .tx_receipt(tx_receipt)
            .build()
            .unwrap();

        // 4. Update the Validator with the most recent state transition
        self.state_transition = state_transition;

        // 5. Build the expected state based on inputs to the state transition
        let expected_state = SimpleStorageStateBuilder::default()
            .value("hi".to_string())
            .last_sender(self.contract.client().address())
            .build()
            .unwrap();

        Ok(expected_state)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_init() {
        let _validator = SimpleStorageValidator::init();
    }
}

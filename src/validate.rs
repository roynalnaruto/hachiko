use ethers::contract::ContractError;

use crate::{State, StateTransition, Validator};

#[allow(dead_code)]
pub async fn validate<S, T, V>(validator: &mut V) -> Result<(), ContractError>
where
    S: State,
    T: StateTransition,
    V: Validator<S, T>,
{
    // 1. Sync the Validator's state
    let initial_state = validator.sync_state().await?;

    // 2. Transition the Validator's state with a transaction
    let expected_state = validator.state_transition(initial_state).await?;

    // 3. Sync the Validator's state
    validator.sync_state().await?;

    // 3. Validator's most recent state should equal the expected state from transition
    assert_eq!(validator.get_state(), expected_state);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use ethers::{
        prelude::*,
        utils::{Ganache, Solc},
    };
    use std::{convert::TryFrom, sync::Arc, time::Duration};

    use crate::{simple_storage_validator::SimpleStorageValidator, ValidatorBase, ValidatorConfig};

    abigen!(SimpleContract, "./contract-abi/SimpleStorage.json");

    #[tokio::test]
    async fn test_validate_deploy() {
        let compiled = Solc::new("./contract-src/SimpleStorage.sol")
            .build()
            .unwrap();
        let contract = compiled
            .get("SimpleStorage")
            .expect("could not find contract");

        // 2. launch ganache
        let ganache = Ganache::new().spawn();

        // 3. instantiate our wallet
        let wallet: Wallet = ganache.keys()[0].clone().into();

        // 4. connect to the network
        let provider = Provider::<Http>::try_from(ganache.endpoint())
            .unwrap()
            .interval(Duration::from_millis(10u64));

        // 5. instantiate the client with the wallet
        let client = wallet.clone().connect(provider);
        let client = Arc::new(client);

        // 6. create a factory which will be used to deploy instances of the contract
        let factory = ContractFactory::new(
            contract.abi.clone(),
            contract.bytecode.clone(),
            client.clone(),
        );

        // 7. deploy it with the constructor arguments
        let deployer = factory.deploy("initial value".to_string()).unwrap();
        let contract = deployer.clone().send().await.unwrap();

        // 8. get validator config
        let validator_config = ValidatorConfig {
            wallet: wallet.clone(),
            address: contract.address(),
            url: ganache.endpoint().into(),
        };

        // 9. create new validator
        let mut validator = SimpleStorageValidator::init_with(validator_config);

        // 10. validate
        validate(&mut validator).await.unwrap();
    }

    #[tokio::test]
    #[ignore = "test only with running ganache with a deployed instance of SimpleStorage"]
    async fn test_validate_dev() {
        let mut validator = SimpleStorageValidator::init();

        validate(&mut validator).await.unwrap();
    }
}

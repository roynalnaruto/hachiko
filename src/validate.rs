use ethers::contract::ContractError;

use crate::{State, StateTransition, Validator};

#[allow(dead_code)]
pub async fn validate<S, T, V>(validator: V) -> Result<(), ContractError>
where
    S: State,
    T: StateTransition,
    V: Validator<S, T>,
{
    let initial_state = validator.before_state().await?;
    println!("initial state = {:?}", initial_state);
    let (state_transition, expected_state) = validator.state_transition(initial_state).await?;
    println!("state transition = {:?}", state_transition);
    let final_state = validator.after_state().await?;
    println!("final state = {:?}", final_state);

    assert_eq!(final_state, expected_state);

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
    async fn test_validate_dev() {
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
        let validator = SimpleStorageValidator::init_with(validator_config);

        // 10. validate
        validate(validator).await.unwrap();
    }
}

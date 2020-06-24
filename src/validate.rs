use crate::{State, StateTransition, Validator};

pub(crate) async fn validate<S, T, V>(validator: V)
where
    S: State,
    T: StateTransition,
    V: Validator<S, T>,
{
    validator.before_state().await;
    validator.state_transition().await;
    validator.after_state().await;
}

#[cfg(test)]
mod test {
    use super::*;
    use ethers::{
        prelude::*,
        utils::{Ganache, Solc},
    };
    use std::{convert::TryFrom, sync::Arc, time::Duration};

    use crate::{simple_storage_validator::SimpleStorageValidator, ValidatorConfig};

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
        let client = wallet.connect(provider);
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
            private_key: ganache.keys()[0].clone(),
            address: contract.address(),
            url: ganache.endpoint().into(),
        };

        // 9. create new validator
        let validator = SimpleStorageValidator::new(validator_config);

        // 10. validate
        validate(validator).await;
    }
}

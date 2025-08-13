use fluentbase_sdk::{derive::router, Address, SharedAPI, U256};
use token_interfaces::{IERC20, IERC20Client};
use shared_types::TokenMetadata;

#[derive(Contract)]
pub struct TokenFactory<SDK> {
    pub sdk: SDK,
}

#[router(mode = "solidity")]
impl<SDK: SharedAPI> TokenFactory<SDK> {
    pub fn create_token(&mut self, name: String, symbol: String) -> Address {
        // TODO: Implement token creation logic
        Address::ZERO
    }
    
    pub fn interact_with_token(&mut self, token: Address, user: Address) -> U256 {
        // TODO: Implement token interaction logic
        U256::zero()
    }
}

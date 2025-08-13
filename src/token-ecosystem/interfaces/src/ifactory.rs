use fluentbase_sdk::{derive::client, Address, U256};
use shared_types::TokenMetadata;

#[client(mode = "solidity")]
pub trait IFactory {
    #[function_id("createToken(string,string)")]
    fn create_token(&mut self, name: String, symbol: String) -> Address;
    
    #[function_id("interactWithToken(address,address)")]
    fn interact_with_token(&self, token: Address, user: Address) -> U256;
}

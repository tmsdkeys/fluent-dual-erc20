use fluentbase_sdk::{derive::client, Address, U256};
use shared_types::TokenMetadata;

#[client(mode = "solidity")]
pub trait IFactory {
    #[function_id("createToken(string,string,uint8)")]
    fn create_token(&mut self, name: String, symbol: String, decimals: u8) -> Address;

    #[function_id("getTokenCount()")]
    fn get_token_count(&self) -> U256;

    #[function_id("getTokenByIndex(uint256)")]
    fn get_token_by_index(&self, index: U256) -> Address;

    #[function_id("getTokenMetadata(address)")]
    fn get_token_metadata(&self, token_address: Address) -> TokenMetadata;

    #[function_id("isToken(address)")]
    fn is_token(&self, token_address: Address) -> bool;
}

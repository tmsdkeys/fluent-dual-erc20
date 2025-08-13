use fluentbase_sdk::{derive::client, Address, U256};
use shared_types::TokenMetadata;

#[client(mode = "solidity")]
pub trait IERC20 {
    #[function_id("name()")]
    fn name(&self) -> String;
    
    #[function_id("symbol()")]
    fn symbol(&self) -> String;
    
    #[function_id("balanceOf(address)")]
    fn balance_of(&self, owner: Address) -> U256;
    
    #[function_id("transfer(address,uint256)")]
    fn transfer(&mut self, to: Address, amount: U256) -> bool;
    
    #[function_id("createWithMetadata((string,string,uint8))")]
    fn create_with_metadata(&mut self, metadata: TokenMetadata) -> Address;
}

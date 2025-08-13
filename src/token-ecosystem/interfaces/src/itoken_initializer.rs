use fluentbase_sdk::{derive::client, Address, Bytes, U256};

/// Separate trait for token initialization - not part of the standard ERC20 interface
#[client(mode = "solidity")]
pub trait ITokenInitializer {
    #[function_id("initialize(string,string,uint8,uint256)")]
    fn initialize(
        &mut self,
        name: String,
        symbol: String,
        decimals: u8,
        total_supply: U256,
    ) -> bool;
}

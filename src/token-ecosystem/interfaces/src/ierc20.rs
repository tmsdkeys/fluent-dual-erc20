use fluentbase_sdk::{derive::client, Address, Bytes, U256};

#[client(mode = "solidity")]
pub trait IERC20 {
    #[function_id("name()")]
    fn name(&self) -> Bytes;

    #[function_id("symbol()")]
    fn symbol(&self) -> Bytes;

    #[function_id("decimals()")]
    fn decimals(&self) -> U256;

    #[function_id("totalSupply()")]
    fn total_supply(&self) -> U256;

    #[function_id("balanceOf(address)")]
    fn balance_of(&self, owner: Address) -> U256;

    #[function_id("transfer(address,uint256)")]
    fn transfer(&mut self, to: Address, amount: U256) -> U256;

    #[function_id("allowance(address,address)")]
    fn allowance(&self, owner: Address, spender: Address) -> U256;

    #[function_id("approve(address,uint256)")]
    fn approve(&mut self, spender: Address, value: U256) -> U256;

    #[function_id("transferFrom(address,address,uint256)")]
    fn transfer_from(&mut self, from: Address, to: Address, value: U256) -> U256;
}

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

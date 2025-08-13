use crate::registry::TokenRegistry;
use fluentbase_sdk::{derive::router, Address, SharedAPI, U256};
use shared_types::{TokenInfo, TokenMetadata};
use token_interfaces::{IERC20Client, IFactory, IERC20};

#[derive(Contract)]
pub struct TokenFactory<SDK> {
    pub sdk: SDK,
}

#[router(mode = "solidity")]
impl<SDK: SharedAPI> IFactory for TokenFactory<SDK> {
    fn create_token(&mut self, name: String, symbol: String, decimals: u8) -> Address {
        // TODO: Implement actual token deployment logic
        // For now, return a placeholder address
        let token_address = Address::ZERO; // This would be the deployed contract address

        // Create metadata for the new token
        let metadata = TokenMetadata {
            name: name.clone(),
            symbol: symbol.clone(),
            decimals,
            total_supply: U256::zero(), // Will be set when token is deployed
            owner: self.sdk.context().contract_caller(),
            created_at: U256::zero(), // TODO: Get current block timestamp
        };

        // Register the token in the registry
        if let Err(_) = TokenRegistry::register_token(&mut self.sdk, token_address, metadata) {
            return Address::ZERO;
        }

        token_address
    }

    fn get_token_count(&self) -> U256 {
        TokenRegistry::get_token_count(&self.sdk)
    }

    fn get_token_by_index(&self, index: U256) -> Address {
        TokenRegistry::get_token_by_index(&self.sdk, index).unwrap_or(Address::ZERO)
    }

    fn get_token_metadata(&self, token_address: Address) -> TokenMetadata {
        TokenRegistry::get_token_info(&self.sdk, token_address)
            .map(|info| info.metadata)
            .unwrap_or_else(|| TokenMetadata {
                name: "Unknown".to_string(),
                symbol: "UNK".to_string(),
                decimals: 18,
                total_supply: U256::zero(),
                owner: Address::ZERO,
                created_at: U256::zero(),
            })
    }

    fn is_token(&self, token_address: Address) -> bool {
        TokenRegistry::is_token(&self.sdk, token_address)
    }
}

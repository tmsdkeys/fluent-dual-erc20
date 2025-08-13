use crate::registry::TokenRegistry;
use fluentbase_sdk::{
    derive::{router, Contract},
    Address, SharedAPI, U256,
};
use shared_types::{TokenInfo, TokenMetadata};
use token_interfaces::{IERC20Client, IFactory, IERC20};

#[derive(Contract)]
pub struct TokenFactory<SDK> {
    pub sdk: SDK,
}

#[router(mode = "solidity")]
impl<SDK: SharedAPI> IFactory for TokenFactory<SDK> {
    fn create_token(&mut self, name: String, symbol: String, decimals: u8) -> Address {
        // TODO: In a real implementation, this would deploy a NEW instance of the ERC20 contract
        // For now, we'll simulate this by using a placeholder address
        // In production, this would use CREATE2 or similar deployment mechanism

        let token_address = Address::ZERO; // Placeholder - would be actual deployed address

        // Create metadata for the new token
        let metadata = TokenMetadata {
            name: name.clone(),
            symbol: symbol.clone(),
            decimals,
            total_supply: U256::zero(), // Will be set during initialization
            owner: self.sdk.context().contract_caller(),
            created_at: U256::zero(), // Would be actual block timestamp
        };

        // Use the ITokenInitializerClient to initialize the newly deployed token
        let mut initializer_client =
            token_interfaces::ITokenInitializerClient::new(self.sdk.clone());

        // Calculate a reasonable total supply (e.g., 1 million tokens with proper decimals)
        let total_supply = U256::from(1_000_000) * U256::from(10).pow(decimals);

        // Initialize the token with the provided parameters
        let success = initializer_client.initialize(
            token_address, // Contract to call
            U256::zero(),  // No ETH value
            200000,        // Gas limit for initialization
            name,          // Token name
            symbol,        // Token symbol
            decimals,      // Token decimals
            total_supply,  // Total supply
        );

        if success {
            // Update metadata with actual values
            let mut updated_metadata = metadata;
            updated_metadata.total_supply = total_supply;

            // Register the token in our registry
            if let Err(_) =
                TokenRegistry::register_token(&mut self.sdk, token_address, updated_metadata)
            {
                return Address::ZERO;
            }

            token_address
        } else {
            Address::ZERO
        }
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

// Additional function demonstrating cross-contract interaction using client generation
impl<SDK: SharedAPI> TokenFactory<SDK> {
    /// Demonstrate cross-contract interaction using IERC20Client
    /// This shows the pattern described in contract-composition.md
    ///
    /// IMPORTANT: This function assumes the token_address is a REAL deployed ERC20 contract
    /// that implements the IERC20 interface. It demonstrates how the factory can
    /// interact with ANY deployed ERC20 token, not just ones it created.
    pub fn interact_with_token(&self, token: Address, user: Address) -> U256 {
        // Create a client to interact with the deployed token
        let token_client = IERC20Client::new(self.sdk.clone());

        // Use the client to call the token's balanceOf function
        // This is the REAL client generation pattern - calling functions on
        // deployed contracts through generated clients
        token_client.balance_of(
            token,        // Contract to call
            U256::zero(), // No ETH value
            50000,        // Gas limit
            user,         // Function parameter
        )
    }
}

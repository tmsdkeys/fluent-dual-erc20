use fluentbase_sdk::Address;
use shared_types::TokenMetadata;

pub struct TokenRegistry {
    // TODO: Implement token tracking logic
}

impl TokenRegistry {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn register_token(&mut self, address: Address, metadata: TokenMetadata) {
        // TODO: Implement token registration
    }
}

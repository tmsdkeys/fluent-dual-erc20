use fluentbase_sdk::{codec::Codec, Address, U256};

#[derive(Debug, Clone, Codec, PartialEq)]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: U256,
    pub owner: Address,
    pub created_at: U256,
}

#[derive(Debug, Clone, Codec, PartialEq)]
pub struct TokenInfo {
    pub address: Address,
    pub metadata: TokenMetadata,
    pub is_active: bool,
}

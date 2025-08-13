use fluentbase_sdk::{derive::solidity_storage, Address, U256};
use shared_types::{TokenInfo, TokenMetadata};

solidity_storage! {
    mapping(Address => TokenInfo) Tokens;
    mapping(U256 => Address) TokenByIndex;
    U256 TokenCount;
}

pub struct TokenRegistry;

impl TokenRegistry {
    pub fn new() -> Self {
        Self {}
    }

    pub fn register_token<SDK: fluentbase_sdk::SharedAPI>(
        sdk: &mut SDK,
        address: Address,
        metadata: TokenMetadata,
    ) -> Result<(), &'static str> {
        let token_count = TokenCount::get(sdk);
        let new_index = token_count + U256::from(1);

        let token_info = TokenInfo {
            address,
            metadata: metadata.clone(),
            is_active: true,
        };

        // Store the token info
        Tokens::set(sdk, address, token_info);

        // Store the index mapping
        TokenByIndex::set(sdk, new_index, address);

        // Update the count
        TokenCount::set(sdk, new_index);

        Ok(())
    }

    pub fn get_token_info<SDK: fluentbase_sdk::SharedAPI>(
        sdk: &SDK,
        address: Address,
    ) -> Option<TokenInfo> {
        let info = Tokens::get(sdk, address);
        if info.address == Address::ZERO {
            None
        } else {
            Some(info)
        }
    }

    pub fn get_token_count<SDK: fluentbase_sdk::SharedAPI>(sdk: &SDK) -> U256 {
        TokenCount::get(sdk)
    }

    pub fn get_token_by_index<SDK: fluentbase_sdk::SharedAPI>(
        sdk: &SDK,
        index: U256,
    ) -> Option<Address> {
        let address = TokenByIndex::get(sdk, index);
        if address == Address::ZERO {
            None
        } else {
            Some(address)
        }
    }

    pub fn is_token<SDK: fluentbase_sdk::SharedAPI>(sdk: &SDK, address: Address) -> bool {
        let info = Tokens::get(sdk, address);
        info.address != Address::ZERO && info.is_active
    }

    pub fn deactivate_token<SDK: fluentbase_sdk::SharedAPI>(
        sdk: &mut SDK,
        address: Address,
    ) -> Result<(), &'static str> {
        let mut info = Tokens::get(sdk, address);
        if info.address == Address::ZERO {
            return Err("Token not found");
        }

        info.is_active = false;
        Tokens::set(sdk, address, info);

        Ok(())
    }
}

#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(dead_code)]
extern crate alloc;
extern crate fluentbase_sdk;

use alloc::vec::Vec;
use alloy_sol_types::{sol, SolEvent};
use fluentbase_sdk::{
    basic_entrypoint,
    derive::{router, solidity_storage, Contract},
    Address, Bytes, ContextReader, SharedAPI, B256, U256,
};

// Import the shared interfaces instead of defining our own
use token_interfaces::{ITokenInitializer, IERC20};

// Define the Transfer and Approval events
sol! {
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
}

fn emit_event<SDK: SharedAPI, T: SolEvent>(sdk: &mut SDK, event: T) {
    let data = event.encode_data();
    let topics: Vec<B256> = event
        .encode_topics()
        .iter()
        .map(|v| B256::from(v.0))
        .collect();
    sdk.emit_log(&topics, &data);
}

solidity_storage! {
    mapping(Address => U256) Balance;
    mapping(Address => mapping(Address => U256)) Allowance;
    // Add storage for token configuration
    String TokenName;
    String TokenSymbol;
    U256 TokenDecimals;
    U256 TokenTotalSupply;
    Address TokenOwner;
    bool IsInitialized;
}

impl Balance {
    fn add<SDK: SharedAPI>(
        sdk: &mut SDK,
        address: Address,
        amount: U256,
    ) -> Result<(), &'static str> {
        let current_balance = Self::get(sdk, address);
        let new_balance = current_balance + amount;
        Self::set(sdk, address, new_balance);
        Ok(())
    }
    fn subtract<SDK: SharedAPI>(
        sdk: &mut SDK,
        address: Address,
        amount: U256,
    ) -> Result<(), &'static str> {
        let current_balance = Self::get(sdk, address);
        if current_balance < amount {
            return Err("insufficient balance");
        }
        let new_balance = current_balance - amount;
        Self::set(sdk, address, new_balance);
        Ok(())
    }
}

impl Allowance {
    fn add<SDK: SharedAPI>(
        sdk: &mut SDK,
        owner: Address,
        spender: Address,
        amount: U256,
    ) -> Result<(), &'static str> {
        let current_allowance = Self::get(sdk, owner, spender);
        let new_allowance = current_allowance + amount;
        Self::set(sdk, owner, spender, new_allowance);
        Ok(())
    }
    fn subtract<SDK: SharedAPI>(
        sdk: &mut SDK,
        owner: Address,
        spender: Address,
        amount: U256,
    ) -> Result<(), &'static str> {
        let current_allowance = Self::get(sdk, owner, spender);
        if current_allowance < amount {
            return Err("insufficient allowance");
        }
        let new_allowance = current_allowance - amount;
        Self::set(sdk, owner, spender, new_allowance);
        Ok(())
    }
}

#[derive(Contract, Default)]
struct ERC20<SDK> {
    sdk: SDK,
}

// Separate deploy function for contract initialization (not part of IERC20 interface)
impl<SDK: SharedAPI> ERC20<SDK> {
    pub fn deploy(&mut self) {
        let owner_address = self.sdk.context().contract_caller();
        let owner_balance: U256 = U256::from_str_radix("1000000000000000000000000", 10).unwrap();

        let _ = Balance::add(&mut self.sdk, owner_address, owner_balance);
    }

    /// Initialize the token with custom parameters
    /// This is called by the factory after deployment
    pub fn initialize(
        &mut self,
        name: String,
        symbol: String,
        decimals: u8,
        total_supply: U256,
    ) -> bool {
        // Check if already initialized
        if IsInitialized::get(&self.sdk) {
            return false;
        }

        let owner = self.sdk.context().contract_caller();

        // Set token configuration
        TokenName::set(&mut self.sdk, name);
        TokenSymbol::set(&mut self.sdk, symbol);
        TokenDecimals::set(&mut self.sdk, U256::from(decimals));
        TokenTotalSupply::set(&mut self.sdk, total_supply);
        TokenOwner::set(&mut self.sdk, owner);
        IsInitialized::set(&mut self.sdk, true);

        // Set initial balance for owner
        let _ = Balance::add(&mut self.sdk, owner, total_supply);

        true
    }
}

#[router(mode = "solidity")]
impl<SDK: SharedAPI> IERC20 for ERC20<SDK> {
    fn symbol(&self) -> Bytes {
        // Return the configured symbol from storage
        let symbol = TokenSymbol::get(&self.sdk);
        Bytes::from(symbol)
    }

    fn name(&self) -> Bytes {
        // Return the configured name from storage
        let name = TokenName::get(&self.sdk);
        Bytes::from(name)
    }

    fn decimals(&self) -> U256 {
        // Return the configured decimals from storage
        TokenDecimals::get(&self.sdk)
    }

    fn total_supply(&self) -> U256 {
        // Return the configured total supply from storage
        TokenTotalSupply::get(&self.sdk)
    }

    fn balance_of(&self, account: Address) -> U256 {
        Balance::get(&self.sdk, account)
    }

    fn transfer(&mut self, to: Address, value: U256) -> U256 {
        let from = self.sdk.context().contract_caller();

        Balance::subtract(&mut self.sdk, from, value).unwrap_or_else(|err| panic!("{}", err));
        Balance::add(&mut self.sdk, to, value).unwrap_or_else(|err| panic!("{}", err));

        emit_event(&mut self.sdk, Transfer { from, to, value });
        U256::from(1)
    }

    fn allowance(&self, owner: Address, spender: Address) -> U256 {
        Allowance::get(&self.sdk, owner, spender)
    }

    fn approve(&mut self, spender: Address, value: U256) -> U256 {
        let owner = self.sdk.context().contract_caller();
        Allowance::set(&mut self.sdk, owner, spender, value);
        emit_event(
            &mut self.sdk,
            Approval {
                owner,
                spender,
                value,
            },
        );
        U256::from(1)
    }

    fn transfer_from(&mut self, from: Address, to: Address, value: U256) -> U256 {
        let spender = self.sdk.context().contract_caller();

        let current_allowance = Allowance::get(&self.sdk, from, spender);
        if current_allowance < value {
            panic!("insufficient allowance");
        }

        Allowance::subtract(&mut self.sdk, from, spender, value)
            .unwrap_or_else(|err| panic!("{}", err));
        Balance::subtract(&mut self.sdk, from, value).unwrap_or_else(|err| panic!("{}", err));
        Balance::add(&mut self.sdk, to, value).unwrap_or_else(|err| panic!("{}", err));

        emit_event(&mut self.sdk, Transfer { from, to, value });
        U256::from(1)
    }
}

// Separate implementation for token initialization
#[router(mode = "solidity")]
impl<SDK: SharedAPI> ITokenInitializer for ERC20<SDK> {
    fn initialize(
        &mut self,
        name: String,
        symbol: String,
        decimals: u8,
        total_supply: U256,
    ) -> bool {
        // Check if already initialized
        if IsInitialized::get(&self.sdk) {
            return false;
        }

        let owner = self.sdk.context().contract_caller();

        // Set token configuration
        TokenName::set(&mut self.sdk, name);
        TokenSymbol::set(&mut self.sdk, symbol);
        TokenDecimals::set(&mut self.sdk, U256::from(decimals));
        TokenTotalSupply::set(&mut self.sdk, total_supply);
        TokenOwner::set(&mut self.sdk, owner);
        IsInitialized::set(&mut self.sdk, true);

        // Set initial balance for owner
        let _ = Balance::add(&mut self.sdk, owner, total_supply);

        true
    }
}

basic_entrypoint!(ERC20);

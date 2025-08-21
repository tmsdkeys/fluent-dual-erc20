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

pub trait ERC20API {
    fn symbol(&self) -> Bytes;
    fn name(&self) -> Bytes;
    fn decimals(&self) -> U256;
    fn total_supply(&self) -> U256;
    fn balance_of(&self, account: Address) -> U256;
    fn transfer(&mut self, to: Address, value: U256) -> U256;
    fn allowance(&self, owner: Address, spender: Address) -> U256;
    fn approve(&mut self, spender: Address, value: U256) -> U256;
    fn transfer_from(&mut self, from: Address, to: Address, value: U256) -> U256;
    fn initialize(&mut self, name: Bytes, symbol: Bytes, initial_supply: U256);
}

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
    Bytes TokenName;
    Bytes TokenSymbol;
    U256 TotalSupply;
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

#[router(mode = "solidity")]
impl<SDK: SharedAPI> ERC20API for ERC20<SDK> {
    fn symbol(&self) -> Bytes {
        self.ensure_initialized();
        TokenSymbol::get(&self.sdk)
    }

    fn name(&self) -> Bytes {
        self.ensure_initialized();
        TokenName::get(&self.sdk)
    }

    fn decimals(&self) -> U256 {
        self.ensure_initialized();
        U256::from(18)
    }

    fn total_supply(&self) -> U256 {
        self.ensure_initialized();
        TotalSupply::get(&self.sdk)
    }

    fn balance_of(&self, account: Address) -> U256 {
        self.ensure_initialized();
        Balance::get(&self.sdk, account)
    }

    fn transfer(&mut self, to: Address, value: U256) -> U256 {
        self.ensure_initialized();
        let from = self.sdk.context().contract_caller();

        Balance::subtract(&mut self.sdk, from, value).unwrap_or_else(|err| panic!("{}", err));
        Balance::add(&mut self.sdk, to, value).unwrap_or_else(|err| panic!("{}", err));

        emit_event(&mut self.sdk, Transfer { from, to, value });
        U256::from(1)
    }

    fn allowance(&self, owner: Address, spender: Address) -> U256 {
        self.ensure_initialized();
        Allowance::get(&self.sdk, owner, spender)
    }

    fn approve(&mut self, spender: Address, value: U256) -> U256 {
        self.ensure_initialized();
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
        self.ensure_initialized();
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

    #[function_id("initialize(bytes,bytes,uint256)")]
    fn initialize(&mut self, name: Bytes, symbol: Bytes, initial_supply: U256) {
        // Check if the token has already been initialized
        // if self.is_initialized() {
        //     panic!("Token already initialized");
        // }

        // Initialize the token with the provided parameters
        TotalSupply::set(&mut self.sdk, initial_supply);
        TokenName::set(&mut self.sdk, name);
        TokenSymbol::set(&mut self.sdk, symbol);

        // Set the initial balance for the caller
        let owner_address = self.sdk.context().contract_caller();
        let _ = Balance::add(&mut self.sdk, owner_address, initial_supply);
    }
}

impl<SDK: SharedAPI> ERC20<SDK> {
    /// Check if the token has been initialized
    fn is_initialized(&self) -> bool {
        TotalSupply::get(&self.sdk) != U256::from(0)
    }

    /// Ensure the token is initialized, panic if not
    fn ensure_initialized(&self) {
        if !self.is_initialized() {
            panic!("Token not initialized");
        }
    }

    /// Public function to check if the token is initialized
    #[function_id("isInitialized()")]
    pub fn is_initialized_public(&self) -> bool {
        self.is_initialized()
    }

    pub fn deploy(&mut self) {
        // Basic deployment - this is called by basic_entrypoint
        // Initialize with default values
        let default_name = Bytes::from("DefaultToken");
        let default_symbol = Bytes::from("DEF");
        let default_supply = U256::from(0);

        TotalSupply::set(&mut self.sdk, default_supply);
        TokenName::set(&mut self.sdk, default_name);
        TokenSymbol::set(&mut self.sdk, default_symbol);
    }
}

basic_entrypoint!(ERC20);

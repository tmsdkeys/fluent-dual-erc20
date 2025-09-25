#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(dead_code)]
extern crate alloc;
extern crate fluentbase_sdk;

use alloc::{string::String, vec::Vec};
use alloy_sol_types::{sol, SolEvent};
use fluentbase_sdk::{
    basic_entrypoint,
    derive::{constructor, router, Contract},
    storage::{StorageMap, StorageString, StorageU256},
    Address, ContextReader, SharedAPI, B256, U256,
};

pub trait ERC20API {
    fn symbol(&self) -> String;
    fn name(&self) -> String;
    fn decimals(&self) -> U256;
    fn total_supply(&self) -> U256;
    fn balance_of(&self, account: Address) -> U256;
    fn transfer(&mut self, to: Address, value: U256) -> U256;
    fn allowance(&self, owner: Address, spender: Address) -> U256;
    fn approve(&mut self, spender: Address, value: U256) -> U256;
    fn transfer_from(&mut self, from: Address, to: Address, value: U256) -> U256;
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

#[derive(Contract)]
struct ERC20<SDK> {
    sdk: SDK,
    token_name: StorageString,
    token_symbol: StorageString,
    token_decimals: StorageU256,
    total_supply: StorageU256,
    balances: StorageMap<Address, StorageU256>,
    allowances: StorageMap<Address, StorageMap<Address, StorageU256>>,
}

#[constructor(mode = "solidity")]
impl<SDK: SharedAPI> ERC20<SDK> {
    pub fn constructor(
        &mut self,
        name: String,
        symbol: String,
        decimals: U256,
        initial_supply: U256,
    ) {
        self.token_name_accessor().set(&mut self.sdk, &name);
        self.token_symbol_accessor().set(&mut self.sdk, &symbol);
        self.token_decimals_accessor().set(&mut self.sdk, decimals);
        self.total_supply_accessor()
            .set(&mut self.sdk, initial_supply);

        let deployer = self.sdk.context().contract_caller();
        self.balances_accessor()
            .entry(deployer)
            .set(&mut self.sdk, initial_supply);

        emit_event(
            &mut self.sdk,
            Transfer {
                from: Address::ZERO,
                to: deployer,
                value: initial_supply,
            },
        );
    }
}

#[router(mode = "solidity")]
impl<SDK: SharedAPI> ERC20API for ERC20<SDK> {
    fn symbol(&self) -> String {
        self.token_symbol_accessor().get(&self.sdk)
    }

    fn name(&self) -> String {
        self.token_name_accessor().get(&self.sdk)
    }

    fn decimals(&self) -> U256 {
        self.token_decimals_accessor().get(&self.sdk)
    }

    fn total_supply(&self) -> U256 {
        self.total_supply_accessor().get(&self.sdk)
    }

    fn balance_of(&self, account: Address) -> U256 {
        self.balances_accessor().entry(account).get(&self.sdk)
    }

    fn transfer(&mut self, to: Address, value: U256) -> U256 {
        let from = self.sdk.context().contract_caller();

        let from_balance = self.balances_accessor().entry(from).get(&self.sdk);
        if from_balance < value {
            panic!("insufficient balance");
        }

        self.balances_accessor()
            .entry(from)
            .set(&mut self.sdk, &from_balance - value);

        let to_balance = self.balances_accessor().entry(to).get(&self.sdk);
        self.balances_accessor()
            .entry(to)
            .set(&mut self.sdk, &to_balance + value);

        emit_event(&mut self.sdk, Transfer { from, to, value });
        U256::from(1)
    }

    fn allowance(&self, owner: Address, spender: Address) -> U256 {
        self.allowances_accessor()
            .entry(owner)
            .entry(spender)
            .get(&self.sdk)
    }

    fn approve(&mut self, spender: Address, value: U256) -> U256 {
        let owner = self.sdk.context().contract_caller();

        self.allowances_accessor()
            .entry(owner)
            .entry(spender)
            .set(&mut self.sdk, value);

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

        let current_allowance = self
            .allowances_accessor()
            .entry(from)
            .entry(spender)
            .get(&self.sdk);
        if current_allowance < value {
            panic!("insufficient allowance");
        }

        let from_balance = self.balances_accessor().entry(from).get(&self.sdk);
        if from_balance < value {
            panic!("insufficient balance");
        }

        self.allowances_accessor()
            .entry(from)
            .entry(spender)
            .set(&mut self.sdk, &current_allowance - value);

        self.balances_accessor()
            .entry(from)
            .set(&mut self.sdk, &from_balance - value);

        let to_balance = self.balances_accessor().entry(to).get(&self.sdk);
        self.balances_accessor()
            .entry(to)
            .set(&mut self.sdk, &to_balance + value);

        emit_event(&mut self.sdk, Transfer { from, to, value });
        U256::from(1)
    }
}

basic_entrypoint!(ERC20);

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

// struct ERC20<SDK: SharedAPI> {
//     sdk: SDK,
// }

// impl<SDK: SharedAPI> ERC20<SDK> {
//     pub fn new(sdk: SDK) -> Self {
//         ERC20 { sdk }
//     }
// }

#[router(mode = "solidity")]
impl<SDK: SharedAPI> ERC20API for ERC20<SDK> {
    fn symbol(&self) -> Bytes {
        Bytes::from("RUSTTK")
    }

    fn name(&self) -> Bytes {
        Bytes::from("RustyToken")
    }

    fn decimals(&self) -> U256 {
        U256::from(18)
    }

    fn total_supply(&self) -> U256 {
        U256::from_str_radix("1000000000000000000000000", 10).unwrap()
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

impl<SDK: SharedAPI> ERC20<SDK> {
    pub fn deploy(&mut self) {
        let owner_address = self.sdk.context().contract_caller();
        let owner_balance: U256 = U256::from_str_radix("1000000000000000000000000", 10).unwrap();

        let _ = Balance::add(&mut self.sdk, owner_address, owner_balance);
    }
}

basic_entrypoint!(ERC20);

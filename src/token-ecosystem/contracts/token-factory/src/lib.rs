#![cfg_attr(target_arch = "wasm32", no_std, no_main)]
extern crate alloc;

mod factory;
mod registry;

use fluentbase_sdk::{basic_entrypoint, derive::Contract};
use factory::TokenFactory;

#[derive(Contract)]
struct TokenFactoryContract<SDK> {
    factory: TokenFactory<SDK>,
}

basic_entrypoint!(TokenFactoryContract);

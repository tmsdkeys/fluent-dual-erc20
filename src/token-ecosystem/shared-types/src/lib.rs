#![cfg_attr(not(feature = "std"), no_std, no_main)]
extern crate alloc;

pub mod token_metadata;

pub use token_metadata::*;

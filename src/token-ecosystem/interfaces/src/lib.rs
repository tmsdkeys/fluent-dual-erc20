#![cfg_attr(not(feature = "std"), no_std, no_main)]
extern crate alloc;

pub mod ierc20;
pub mod ifactory;
pub mod itoken_initializer;

pub use ierc20::*;
pub use ifactory::*;
pub use itoken_initializer::*;

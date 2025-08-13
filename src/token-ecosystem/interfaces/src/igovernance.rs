use fluentbase_sdk::{derive::client, Address, U256};
use shared_types::Proposal;

#[client(mode = "solidity")]
pub trait IGovernance {
    #[function_id("executeProposal(address,address,uint256)")]
    fn execute_proposal(&mut self, factory_addr: Address, token_addr: Address, proposal_id: U256) -> bool;
}

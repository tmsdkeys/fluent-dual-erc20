use fluentbase_sdk::{derive::client, Address, U256};
use shared_types::Proposal;

#[client(mode = "solidity")]
pub trait IGovernance {
    #[function_id("createProposal(string)")]
    fn create_proposal(&mut self, description: String) -> U256;

    #[function_id("executeProposal(uint256)")]
    fn execute_proposal(&mut self, proposal_id: U256) -> bool;

    #[function_id("getProposal(uint256)")]
    fn get_proposal(&self, proposal_id: U256) -> Proposal;

    #[function_id("getProposalCount()")]
    fn get_proposal_count(&self) -> U256;

    #[function_id("vote(uint256,bool)")]
    fn vote(&mut self, proposal_id: U256, support: bool) -> bool;
}

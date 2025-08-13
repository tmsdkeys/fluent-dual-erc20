use fluentbase_sdk::{Address, U256};

#[derive(Debug, Clone)]
pub struct Proposal {
    pub id: U256,
    pub description: String,
    pub creator: Address,
    pub created_at: U256,
    pub executed: bool,
    pub executed_at: Option<U256>,
    pub for_votes: U256,
    pub against_votes: U256,
    pub total_votes: U256,
}

#[derive(Debug, Clone)]
pub struct Vote {
    pub voter: Address,
    pub proposal_id: U256,
    pub support: bool,
    pub weight: U256,
    pub voted_at: U256,
}

#[derive(Debug, Clone)]
pub enum ProposalState {
    Active,
    Executed,
    Cancelled,
    Expired,
}

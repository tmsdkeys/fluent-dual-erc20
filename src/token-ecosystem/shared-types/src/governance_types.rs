use fluentbase_sdk::U256;

#[derive(Debug, Clone)]
pub struct Proposal {
    pub id: U256,
    pub description: String,
    pub executed: bool,
}

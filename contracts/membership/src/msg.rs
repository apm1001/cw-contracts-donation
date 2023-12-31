use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Addr};

#[cw_serde]
pub struct InstantiateMsg {
    pub starting_weight: u64,
    pub denom: String,
    pub direct_part: Decimal,
    pub halftime: u64,
    pub minimal_acceptances: u64,
    pub proxy_code_id: u64,
    pub distribution_code_id: u64,
    pub initial_members: Vec<String>,
}

#[cw_serde]
pub enum ExecMsg {
    ProposeMember { addr: String },
}

#[cw_serde]
pub struct ProposeMemberData {
    pub owner_addr: Addr,
    pub proxy_addr: Addr,
}
 
#[cw_serde]
pub struct InstantationData {
    pub members: Vec<ProposeMemberData>,
}
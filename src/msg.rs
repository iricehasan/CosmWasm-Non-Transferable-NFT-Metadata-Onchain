use cosmwasm_std::Addr;
use cosmwasm_schema::cw_serde;
use cw721_base::Extension;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub name: String,
    pub symbol: String,
    pub cw721_code_id: u64,
    pub token_uri: String,
    pub extension: Extension,
}

#[cw_serde]
pub enum ExecuteMsg {
    Mint {}
}

#[cw_serde]
pub enum QueryMsg {
    Config {}
}
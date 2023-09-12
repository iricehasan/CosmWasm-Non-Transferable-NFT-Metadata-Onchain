use cosmwasm_schema::cw_serde;
use cw_storage_plus::Item;

use cosmwasm_std::Addr;

use cw721_base::Extension;

#[cw_serde]
pub struct Config {
    pub cw721_address: Option<Addr>,
    pub token_id: u64,
    pub token_uri: String,
    pub extension: Extension,
}

pub const CONFIG: Item<Config> = Item::new("config");
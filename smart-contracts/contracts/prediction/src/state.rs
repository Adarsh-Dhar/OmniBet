use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Pool {
    pub id: u64,
    pub creator: Addr,
    pub token: Addr,
    pub liquidity: i128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Market {
    pub id: u64,
    pub question: String,
    pub end_time: u64,
    pub yes_votes: i128,
    pub no_votes: i128,
    pub resolved: bool,
    pub outcome: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Vote {
    pub user: Addr,
    pub amount: i128,
    pub prediction: bool,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const POOLS: Map<u64, Pool> = Map::new("pools");
pub const MARKETS: Map<u64, Market> = Map::new("markets");
pub const VOTES: Map<(u64, Addr), Vote> = Map::new("votes");
pub const POOL_COUNT: Item<u64> = Item::new("pool_count");
pub const MARKET_COUNT: Item<u64> = Item::new("market_count");
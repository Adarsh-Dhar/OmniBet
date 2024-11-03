use std::time::Duration;
use cosmwasm_std::{Addr, Uint128, Timestamp, Binary, Coin};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use crate::state::Bet;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    CreatePool {
        date: Timestamp,
        token: String,
        amount: Uint128,
    },
    EnterBet {
        id: Uint128,
        amount: Uint128,
        bet: Uint128,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetAllPool {},
    GetPoolByToken { token: String },
    GetPoolByDate { date: Timestamp },
}

// Define specific response types for each query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AllPoolsResponse {
    pub pools: Vec<Bet>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PoolsByTokenResponse {
    pub pools: Vec<Bet>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PoolsByDateResponse {
    pub pools: Vec<Bet>,
}



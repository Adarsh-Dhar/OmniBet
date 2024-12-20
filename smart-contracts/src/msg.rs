use std::time::Duration;
use cosmwasm_std::{Addr, Uint128, Timestamp, Binary, Coin, Decimal};
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
        start_date: Uint128,
        end_date: Uint128,
        token: String,
        amount: Uint128,
        deadline: Uint128,
    },
    EnterBet {
        id: Uint128,
        current_date: Uint128,
        bet: Decimal,
    },
    ClaimBet {
        bet_id: Uint128,
        current_date: Uint128,
        real_value: Decimal,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetAllPool {current_time: Uint128},
    GetPoolByToken { token: String, current_time: Uint128 },
    GetPoolByDate { date: Uint128, current_time: Uint128 },
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



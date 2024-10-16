use serde::{Deserialize, Serialize};
use cosmwasm_std::Addr;
use cosmwasm_schema::{cw_serde, QueryResponses};
use schemars::JsonSchema;



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: Option<String>,
    pub min_bet_amount: i128,
}

#[cw_serde]
pub enum ExecuteMsg {
    CreatePool { token: Addr },
    AddLiquidity { pool_id: u64, amount: i128 },
    CreateMarket { question: String, end_time: u64 },
    PlaceVote { market_id: u64, prediction: bool, amount: i128 },
    ResolveMarket { market_id: u64, outcome: bool },
}

#[cw_serde]
pub enum QueryMsg {
    GetConfig {},
    GetPool { id: u64 },
    GetMarket { id: u64 },
    GetVote { market_id: u64, user: Addr },
}


use std::time::Duration;
use cosmwasm_std::{Addr, Uint128, Timestamp, Binary, Coin};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;



use cosmwasm_schema::{
    cw_serde,
    QueryResponses,
};

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreatePool {
        date : Timestamp,
        token : String,
        amount : Uint128,
    },
    EnterBet {
        id : Uint128,
        amount : Uint128,
        bet : Uint128,
},
    // ClaimBet {
    //     bet_id : Uint128,
    //     player : Addr
    // },
   
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct PoolResponse {
//     // Add the fields you want to return in your query
//     pub total_pools: u64,
//     // Add other fields as needed
// }

// #[cw_serde]
// #[derive(QueryResponses)]
// pub enum QueryMsg {
//     #[returns(PoolResponse)]
//     GetAllPool {},
//     #[returns(PoolResponse)]
//     GetPoolByToken { token: String },
//     #[returns(PoolResponse)]
//     GetPoolByDate { date: Timestamp },
// }



use std::time::Duration;
use cosmwasm_std::{Addr, Uint128, Timestamp};

use cosmwasm_std::{Binary, Coin};


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
        owner : Addr,
        deadline : Timestamp,
        token : String,
        amount : Uint128,
    },
    EnterBet {
        id : Uint128,
        amount : Uint128,
        player : Addr,
},
    ClaimBet {
        bet_id : Uint128,
        player : Addr
    },
   
}

#[cw_serde]
pub enum QueryMsg {
    GetAllPool {},
    GetPoolByToken {
        token : String
    },
    GetPoolByDate {
        date : Timestamp
    },
}



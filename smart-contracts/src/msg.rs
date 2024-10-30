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
        amount : Uint128,
        price : Uint128,
        price_key : Addr
    },
    EnterBet {
        amount : Uint128,
        expiry : Timestamp
    },
    ClaimBet {
        palyer : Addr
    },
    CloseBet {
        player : Addr
    }
}



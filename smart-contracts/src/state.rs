use cosmwasm_std::{Addr, Uint128, Timestamp};
use schemars::JsonSchema;
use serde::{
    Deserialize,
    Serialize,
};

use cw_storage_plus::{Item, Map};



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BetPrediction {
    pub player: Addr,
    pub bet: Uint128,
    pub bet_id : Uint128,
}



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Bet {
    pub id : Uint128,
    pub creator : Addr,
    pub amount: Uint128,
    pub state : BetStatus,
    pub expiry : Timestamp,
    pub asset_name : String,
    pub winner : Addr,
}

pub struct Prediction<'a> {
    pub prediction : Map<'a, String, BetPrediction>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum BetStatus {
    not_created,
    created,
    ended,
    claimed,
}


pub const BET: Item<Bet> = Item::new("bet");
pub const BET_PREDICTION: Item<BetPrediction> = Item::new("bet_prediction");


